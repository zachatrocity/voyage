use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdMail, LdMap, LdSettings};
use dioxus_free_icons::Icon;

use crate::components::discovery_banner::DiscoveryBanner;
use crate::components::email_list_item::EmailListItem;
use crate::components::filter_chips::FilterChips;
use crate::components::search_bar::SearchBar;
use crate::types::Category;
use crate::{EMAILS, SELECTED_EMAIL};

#[component]
pub fn EmailList() -> Element {
    let mut search = use_signal(|| String::new());
    let mut active_filter = use_signal(|| "All".to_string());

    let filtered = use_memo(move || {
        let emails = EMAILS.read();
        emails
            .iter()
            .filter(|e| {
                let q = search().to_lowercase();
                let matches_search = q.is_empty()
                    || e.subject.to_lowercase().contains(&q)
                    || e.sender.to_lowercase().contains(&q);
                let matches_filter = match active_filter().as_str() {
                    "Flights ✈️" => e.category == Category::Flight,
                    "Hotels 🏨" => e.category == Category::Hotel,
                    "Car Rental 🚗" => e.category == Category::CarRental,
                    "Cruises 🚢" => e.category == Category::Cruise,
                    "Other" => e.category == Category::Other || e.category == Category::Activity,
                    _ => true, // "All"
                };
                matches_search && matches_filter
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let unreviewed_count = use_memo(move || {
        EMAILS.read().iter().filter(|e| e.trip_id.is_none()).count()
    });

    rsx! {
        div { class: "flex flex-col h-screen bg-background",
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
            div { class: "flex-1 overflow-y-auto py-2 pb-24",
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

            // Bottom nav bar
            div { class: "fixed bottom-0 left-0 right-0 bg-card border-t border-border px-6 py-2 flex justify-around items-center",
                button { class: "flex flex-col items-center gap-0.5 text-primary",
                    Icon { icon: LdMail, width: 20, height: 20 }
                    span { class: "text-xs font-medium", "Emails" }
                }
                button { class: "flex flex-col items-center gap-0.5 text-muted",
                    Icon { icon: LdMap, width: 20, height: 20 }
                    span { class: "text-xs", "Trips" }
                }
                button { class: "flex flex-col items-center gap-0.5 text-muted",
                    Icon { icon: LdSettings, width: 20, height: 20 }
                    span { class: "text-xs", "Settings" }
                }
            }
        }
    }
}
