use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdArrowLeft;
use dioxus_free_icons::Icon;
use gloo_timers::future::TimeoutFuture;

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
    let mut emails = use_signal(|| Vec::<Email>::new());
    let mut loading = use_signal(|| true);
    let mut unreviewed_count = use_signal(|| 0usize);

    // Debounced query that triggers API calls
    let mut debounced_query = use_signal(|| String::new());

    // Debounce: when search changes, wait 300ms then update debounced_query
    use_effect(move || {
        let query = search().clone();
        spawn(async move {
            TimeoutFuture::new(300).await;
            // Only update if search hasn't changed during the wait
            if search() == query {
                debounced_query.set(query);
            }
        });
    });

    // Fetch emails when debounced_query changes
    let _email_resource = use_resource(move || {
        let query = debounced_query().clone();
        async move {
            loading.set(true);
            match api::search_emails(&query, Some(50)).await {
                Ok(results) => {
                    let converted: Vec<Email> =
                        results.emails.into_iter().map(Email::from).collect();
                    emails.set(converted);
                }
                Err(e) => {
                    notify_error(format!("Failed to load emails: {e}"));
                    emails.set(Vec::new());
                }
            }
            loading.set(false);
        }
    });

    // Fetch untagged travel email count for discovery banner
    let _untagged_resource = use_resource(move || async move {
        match api::search_emails("", Some(50)).await {
            Ok(results) => {
                let count = results.emails.iter().filter(|e| e.trip_id.is_none()).count();
                unreviewed_count.set(count);
            }
            Err(_) => {
                unreviewed_count.set(0);
            }
        }
    });

    // Client-side category filter
    let filtered = use_memo(move || {
        let all = emails.read();
        all.iter()
            .filter(|e| {
                match active_filter().as_str() {
                    "Flights ✈️" => e.category == Category::Flight,
                    "Hotels 🏨" => e.category == Category::Hotel,
                    "Car Rental 🚗" => e.category == Category::CarRental,
                    "Cruises 🚢" => e.category == Category::Cruise,
                    "Other" => e.category == Category::Other || e.category == Category::Activity,
                    _ => true, // "All"
                }
            })
            .cloned()
            .collect::<Vec<_>>()
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
                value: search(),
                on_change: move |v: String| search.set(v),
            }

            DiscoveryBanner { count: unreviewed_count() }

            FilterChips {
                active: active_filter(),
                on_change: move |v: String| active_filter.set(v),
            }

            // Email list
            div { class: "flex-1 overflow-y-auto py-2 pb-4",
                if loading() {
                    div { class: "flex flex-col items-center justify-center py-12 text-muted",
                        div { class: "w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin mb-3" }
                        span { class: "text-sm", "Loading emails..." }
                    }
                } else if filtered().is_empty() {
                    div { class: "flex flex-col items-center justify-center py-12 text-muted",
                        span { class: "text-4xl mb-2", "📭" }
                        span { class: "text-sm", "No emails match your search" }
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
                }
            }
        }
    }
}
