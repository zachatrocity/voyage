use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;

use crate::api;
use crate::components::discovery_banner::DiscoveryBanner;
use crate::components::email_list_item::EmailListItem;
use crate::components::filter_chips::FilterChips;
use crate::components::search_bar::SearchBar;
use crate::notification::notify_error;
use crate::types::{Category, Email};
use crate::SELECTED_EMAIL;

#[component]
pub fn EmailList() -> Element {
    let mut search = use_signal(|| String::new());
    let mut active_filter = use_signal(|| "All".to_string());

    // Fetch emails reactively based on search query
    let email_resource = use_resource(move || {
        let query = search().clone();
        async move {
            let client = match api::get_client() {
                Ok(c) => c,
                Err(e) => {
                    notify_error(format!("Config error: {e}"));
                    return Vec::new();
                }
            };
            // Backend requires non-empty q param
            let q = if query.is_empty() { "*".to_string() } else { query };
            match client.search_emails(Some("50"), &q, None::<&str>).await {
                Ok(resp) => {
                    let results = resp.into_inner();
                    results.results.into_iter().map(Email::from).collect::<Vec<_>>()
                }
                Err(e) => {
                    notify_error(format!("Failed to load emails: {e}"));
                    Vec::new()
                }
            }
        }
    });

    // Clone data out of the resource signal immediately so the read guard
    // is dropped before rsx! — holding it across the render causes a borrow
    // panic when search triggers a resource restart.
    let loading = email_resource.value().read().is_none();
    let email_list: Vec<Email> = email_resource
        .value()
        .read()
        .clone()
        .unwrap_or_default();

    let unreviewed_count = email_list.iter().filter(|e| e.trip_id.is_none()).count();

    let filtered: Vec<_> = email_list
        .iter()
        .filter(|e| match active_filter().as_str() {
            "Flights ✈️" => e.category == Category::Flight,
            "Hotels 🏨" => e.category == Category::Hotel,
            "Car Rental 🚗" => e.category == Category::CarRental,
            "Cruises 🚢" => e.category == Category::Cruise,
            "Other" => e.category == Category::Other || e.category == Category::Activity,
            _ => true,
        })
        .cloned()
        .collect();

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

            DiscoveryBanner { count: unreviewed_count }

            FilterChips {
                active: active_filter(),
                on_change: move |v: String| active_filter.set(v),
            }

            // Email list
            div { class: "flex-1 overflow-y-auto py-2 pb-4",
                if loading {
                    div { class: "flex flex-col items-center justify-center py-12 text-muted",
                        div { class: "w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin mb-3" }
                        span { class: "text-sm", "Loading emails..." }
                    }
                } else if filtered.is_empty() {
                    div { class: "flex flex-col items-center justify-center py-12 text-muted",
                        span { class: "text-4xl mb-2", "📭" }
                        span { class: "text-sm", "No emails match your search" }
                    }
                } else {
                    for email in filtered.iter() {
                        EmailListItem {
                            key: "{email.id}",
                            email: email.clone(),
                            on_click: move |id: String| {
                                *SELECTED_EMAIL.write() = Some(id);
                            },
                        }
                    }
                }
            }
        }
    }
}
