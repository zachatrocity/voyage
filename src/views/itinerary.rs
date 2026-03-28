use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dioxus_free_icons::Icon;

use crate::components::hero_header::HeroHeader;
use crate::components::timeline_item::TimelineItem;
use crate::types::Trip;
use crate::{ITINERARY, SELECTED_TRIP, TRIPS};

#[component]
pub fn Itinerary() -> Element {
    let trip: Memo<Trip> = use_memo(move || {
        let trips = TRIPS.read();
        let selected = SELECTED_TRIP.read();
        selected
            .as_ref()
            .and_then(|id| trips.iter().find(|t| &t.id == id).cloned())
            .unwrap_or_else(|| trips.first().cloned().unwrap())
    });

    let items = use_memo(move || {
        let all = ITINERARY.read();
        let t = trip();
        let filtered: Vec<_> = all.iter().filter(|i| i.trip_id == t.id).cloned().collect();
        filtered
    });

    rsx! {
        div { class: "flex flex-col h-screen bg-background",
            HeroHeader { trip: trip() }

            div { class: "flex-1 overflow-y-auto px-4 pt-4 pb-24",
                {
                    let all_items = items();
                    let mut prev_date = String::new();
                    let rendered: Vec<_> = all_items.iter().map(|item| {
                        let show_date = item.date != prev_date;
                        prev_date = item.date.clone();
                        rsx! {
                            TimelineItem { key: "{item.id}", item: item.clone(), show_date: show_date }
                        }
                    }).collect();
                    rsx! { {rendered.into_iter()} }
                }
            }

            // FAB
            div { class: "fixed bottom-20 right-4",
                button { class: "w-14 h-14 rounded-full bg-cta shadow-lg flex items-center justify-center",
                    Icon { icon: LdPlus, width: 24, height: 24, class: "text-white" }
                }
            }
        }
    }
}
