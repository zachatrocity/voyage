use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use gloo_timers::future::TimeoutFuture;

use crate::api::{self, ApiError};
use crate::components::discovery_banner::DiscoveryBanner;
use crate::components::email_list_item::EmailListItem;
use crate::components::filter_chips::FilterChips;
use crate::components::search_bar::SearchBar;
use crate::types::{Category, Email};
use crate::SELECTED_EMAIL;

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

fn to_ui_email(e: &api::EmailResult) -> Email {
    Email {
        id: e.id.clone(),
        subject: e.subject.clone(),
        sender: e.sender.clone(),
        sender_email: e.sender_email.clone(),
        date: e.date.clone(),
        body_preview: e.body_preview.clone(),
        category: e.category.clone(),
        trip_id: e.trip_id.clone(),
    }
}

#[component]
pub fn EmailList() -> Element {
    let mut search = use_signal(String::new);
    let mut active_filter = use_signal(|| "All".to_string());

    let emails_resource = use_resource(move || {
        let query = search();
        async move {
            // debounce search input
            TimeoutFuture::new(300).await;
            api::search_emails(&query, Some(50)).await
        }
    });

    let discovery_resource = use_resource(|| async move {
        api::search_emails("", Some(20))
            .await
            .map(|r| r.emails.into_iter().filter(|e| e.trip_id.is_none()).count())
    });

    let filtered = use_memo(move || match &*emails_resource.read_unchecked() {
        Some(Ok(result)) => result
            .emails
            .iter()
            .filter(|e| filter_matches(&e.category, &active_filter()))
            .map(to_ui_email)
            .collect::<Vec<Email>>(),
        _ => Vec::new(),
    });

    let discovery_count = use_memo(move || match &*discovery_resource.read_unchecked() {
        Some(Ok(count)) => *count,
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

    let is_loading = use_memo(move || emails_resource.read_unchecked().is_none());

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
                value: search(),
                on_change: move |v: String| search.set(v),
            }

            DiscoveryBanner { count: discovery_count() }

            FilterChips {
                active: active_filter(),
                on_change: move |v: String| active_filter.set(v),
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
