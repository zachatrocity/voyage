use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::components::discovery_banner::DiscoveryBanner;
use crate::components::email_list_item::EmailListItem;
use crate::components::filter_chips::{FilterChip, FilterChips};
use crate::components::search_bar::SearchBar;
use crate::types::{Category, Email};
use crate::Route;
use crate::{CLASSIFIERS, EMAIL_LIST_FILTER, EMAIL_LIST_QUERY, SELECTED_EMAIL};

fn category_key(category: &Category) -> &'static str {
    match category {
        Category::Flight => "flight",
        Category::Hotel => "hotel",
        Category::CarRental => "car_rental",
        Category::Cruise => "cruise",
        Category::Activity => "activity",
        Category::Other => "other",
    }
}

fn filter_matches(category: &Category, active_filter: &str) -> bool {
    active_filter == "all" || category_key(category) == active_filter
}

fn fallback_title_for_category(key: &str) -> String {
    match key {
        "flight" => "Flights ✈️".to_string(),
        "hotel" => "Hotels 🏨".to_string(),
        "car_rental" => "Car Rental 🚗".to_string(),
        "cruise" => "Cruises 🚢".to_string(),
        "activity" => "Activities 🎟️".to_string(),
        "other" => "Other 📧".to_string(),
        _ => key
            .split('_')
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + chars.as_str()
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn chip_filters() -> Vec<FilterChip> {
    let mut out = vec![FilterChip {
        key: "all".to_string(),
        label: "All".to_string(),
    }];

    let cfg = CLASSIFIERS();
    let Some(cfg) = cfg.as_ref() else {
        out.push(FilterChip {
            key: "flight".to_string(),
            label: fallback_title_for_category("flight"),
        });
        out.push(FilterChip {
            key: "hotel".to_string(),
            label: fallback_title_for_category("hotel"),
        });
        out.push(FilterChip {
            key: "car_rental".to_string(),
            label: fallback_title_for_category("car_rental"),
        });
        out.push(FilterChip {
            key: "cruise".to_string(),
            label: fallback_title_for_category("cruise"),
        });
        out.push(FilterChip {
            key: "activity".to_string(),
            label: fallback_title_for_category("activity"),
        });
        return out;
    };

    let preferred_order = ["flight", "hotel", "car_rental", "cruise", "activity", "other"];
    for key in preferred_order {
        if cfg.categories.contains_key(key) {
            out.push(FilterChip {
                key: key.to_string(),
                label: cfg
                    .category_titles
                    .get(key)
                    .cloned()
                    .unwrap_or_else(|| fallback_title_for_category(key)),
            });
        }
    }

    let mut extras = cfg
        .categories
        .keys()
        .filter(|k| !preferred_order.contains(&k.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    extras.sort();

    for key in extras {
        out.push(FilterChip {
            label: cfg
                .category_titles
                .get(&key)
                .cloned()
                .unwrap_or_else(|| fallback_title_for_category(&key)),
            key,
        });
    }

    out
}

/// Build a search query from a classifier category's subject keywords.
fn query_from_classifier(category_key: &str) -> Option<String> {
    let classifiers = CLASSIFIERS();
    let cfg = classifiers.as_ref()?;
    let rule = cfg.categories.get(category_key)?;
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
                value: EMAIL_LIST_QUERY(),
                on_change: move |v: String| {
                    // When the user types, reset filter to All so the manual
                    // query drives search without category restriction.
                    if EMAIL_LIST_FILTER() != "all" {
                        *EMAIL_LIST_FILTER.write() = "all".to_string();
                    }
                    *EMAIL_LIST_QUERY.write() = v;
                },
            }

            DiscoveryBanner { count: discovery_count() }

            FilterChips {
                active: EMAIL_LIST_FILTER(),
                filters: chip_filters(),
                on_change: move |v: String| {
                    *EMAIL_LIST_FILTER.write() = v.clone();
                    if v != "all" {
                        if let Some(q) = query_from_classifier(&v) {
                            *EMAIL_LIST_QUERY.write() = q;
                        }
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
