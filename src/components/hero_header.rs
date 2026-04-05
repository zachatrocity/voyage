use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdShare2};
use dioxus_free_icons::Icon;

use crate::{types::Trip, Route};

#[component]
pub fn HeroHeader(trip: Trip) -> Element {
    let navigator = use_navigator();
    rsx! {
        div { class: "relative bg-primary px-4 pt-10 pb-5 overflow-hidden",
            // Subtle texture emoji
            div { class: "absolute -right-4 -top-4 text-[100px] opacity-10 select-none pointer-events-none",
                "🗺️"
            }

            // Back + share row
            div { class: "flex items-center justify-between mb-4",
                button {
                    class: "text-white/80",
                    onclick: move |_| { navigator.push(Route::Trips {}); },
                    Icon { icon: LdArrowLeft, width: 20, height: 20 }
                }
                Icon { icon: LdShare2, width: 20, height: 20, class: "text-white/80" }
            }

            // Trip name
            h1 { class: "text-2xl font-bold text-white mb-1", "{trip.name}" }
            p { class: "text-sm text-white/70 mb-3", "✈️ {trip.date_range}" }

            // Stat badges
            div { class: "flex gap-2",
                span { class: "bg-white/20 text-white text-xs rounded-full px-3 py-1",
                    "{trip.email_count} emails tagged"
                }
                span { class: "bg-white/20 text-white text-xs rounded-full px-3 py-1",
                    "{trip.confirmed_count} confirmed"
                }
            }
        }
    }
}
