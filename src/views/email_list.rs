use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::components::discovery_banner::DiscoveryBanner;
use crate::components::email_list_item::EmailListItem;
use crate::components::filter_chips::FilterChips;
use crate::components::search_bar::SearchBar;
use crate::types::{Category, Email};
use crate::Route;
use crate::{CLASSIFIERS, EMAIL_LIST_FILTER, EMAIL_LIST_QUERY, SELECTED_EMAIL};

fn filter_matches(category: &Category, active_filter: &str) -> bool {
    match active_filter {
        "Flights ✈️" => *category == Category::Flight,
        "Hotels 🏨" => *category == Category::Hotel,
        "Car Rental 🚗" => *category == Category::CarRental,
        "Cruises 🚢" => *category == Category::Cruise,
        "Other" => *category == Category::Other || *category == Category::Activity,
        _ => true, // "All"
    }
}

/// Map a filter-chip label to the classifier category key used by the backend.
fn pill_to_classifier_key(pill: &str) -> Option<&'static str> {
    match pill {
        "Flights ✈️" => Some("flights"),
        "Hotels 🏨" => Some("hotels"),
        "Car Rental 🚗" => Some("car_rental"),
        "Cruises 🚢" => Some("cruises"),
        "Other" => Some("other"),
        _ => None,
    }
}

/// Build a search query from a classifier category's subject keywords.
fn query_from_classifier(filter: &str) -> Option<String> {
    let key = pill_to_classifier_key(filter)?;
    let classifiers = CLASSIFIERS();
    let cfg = classifiers.as_ref()?;
    let rule = cfg.categories.get(key)?;
    if rule.subject_keywords.is_empty() {
        return None;
    }
    Some(rule.subject_keywords.join(" OR "))
}

fn to_ui_email(e: &api::EmailResult) -> Email {
    Email {
        id: e.id.clone(),
        subject: e.subject.clone(),
        sender: e.sender.clone(),
        sender_email: e.sender_email.clone(),
        date: e.date.clone(),
        body_preview: e.body_preview.clone(),
        category: e.category.clone(),
        tags: e.tags.clone(),
        trip_id: e.trip_id.clone(),
    }
}

#[component]
pub fn EmailList() -> Element {
    let navigator = use_navigator();
    let mut manual_query = use_signal(String::new);

    // Load classifiers once on mount so pill taps can use their terms.
    use_effect(move || {
        if CLASSIFIERS().is_none() {
            spawn(async move {
                if let Ok(cfg) = api::get_classifiers().await {
                    *CLASSIFIERS.write() = Some(cfg);
                }
            });
        }
    });

    let emails_resource = use_resource(move || {
        let query = EMAIL_LIST_QUERY();
        async move {
            if query.trim().is_empty() {
                return Ok(api::SearchResults {
                    emails: vec![],
                    total: 0,
                });
            }
            api::search_emails(&query, Some(50)).await
        }
    });

    let filtered = use_memo(move || match &*emails_resource.read_unchecked() {
        Some(Ok(result)) => result
            .emails
            .iter()
            .filter(|e| filter_matches(&e.category, &EMAIL_LIST_FILTER()))
            .map(to_ui_email)
            .collect::<Vec<Email>>(),
        _ => Vec::new(),
    });

    let discovery_count = use_memo(move || match &*emails_resource.read_unchecked() {
        Some(Ok(result)) => result.emails.iter().filter(|e| e.trip_id.is_none()).count(),
        _ => 0,
    });

    let search_error = use_memo(move || match &*emails_resource.read_unchecked() {
        Some(Err(err)) => Some(match err {
            ApiError::Network(msg) => format!("Network error: {msg}"),
            ApiError::Decode(msg) => format!("Decode error: {msg}"),
            ApiError::Server { status, message } => format!("Server error ({status}): {message}"),
        }),
        _ => None,
    });

    let is_loading = use_memo(move || {
        if EMAIL_LIST_QUERY().trim().is_empty() {
            return false;
        }
        emails_resource.read_unchecked().is_none()
    });

    rsx! {
        div { class: "flex flex-col h-full bg-background",
            // Header
            div { class: "bg-card border-b border-border px-4 py-3 flex items-center gap-3",
                button { class: "text-muted",
                    Icon { icon: LdArrowLeft, width: 20, height: 20 }
                }
                span { class: "text-lg font-semibold text-foreground flex-1",
                    "Voyage ✈️"
                }
            }

            SearchBar {
                value: manual_query(),
                on_change: move |v: String| {
                    manual_query.set(v.clone());
                    // When the user types, reset filter to All so the manual
                    // query drives search without category restriction.
                    if EMAIL_LIST_FILTER() != "All" {
                        *EMAIL_LIST_FILTER.write() = "All".to_string();
                    }
                    *EMAIL_LIST_QUERY.write() = v;
                },
            }

            DiscoveryBanner { count: discovery_count() }

            FilterChips {
                active: EMAIL_LIST_FILTER(),
                on_change: move |v: String| {
                    *EMAIL_LIST_FILTER.write() = v.clone();
                    if v == "All" {
                        // Restore whatever the user had typed manually.
                        *EMAIL_LIST_QUERY.write() = manual_query();
                    } else if let Some(q) = query_from_classifier(&v) {
                        *EMAIL_LIST_QUERY.write() = q;
                    }
                },
            }

            div { class: "flex-1 overflow-y-auto py-2 pb-4",
                if is_loading() {
                    for i in 0..4 {
                        div {
                            key: "skeleton-{i}",
                            class: "mx-4 mb-2 rounded-xl border border-border bg-card px-4 py-4 animate-pulse",
                            div { class: "h-4 w-2/3 bg-border rounded mb-2" }
                            div { class: "h-3 w-1/2 bg-border rounded" }
                        }
                    }
                } else if let Some(err) = search_error() {
                    div { class: "mx-4 mt-3 rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700",
                        "{err}"
                    }
                } else {
                    for email in filtered().iter() {
                        EmailListItem {
                            key: "{email.id}",
                            email: email.clone(),
                            on_click: move |id: String| {
                                *SELECTED_EMAIL.write() = Some(id);
                                navigator.push(Route::EmailDetail {});
                            },
                        }
                    }

                    if filtered().is_empty() {
                        div { class: "flex flex-col items-center justify-center py-12 text-muted",
                            span { class: "text-4xl mb-2", "📭" }
                            span { class: "text-sm", "No emails match your search" }
                        }
                    }
                }
            }
        }
    }
}
