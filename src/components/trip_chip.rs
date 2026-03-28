use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdCheck;
use dioxus_free_icons::Icon;

use crate::types::Trip;

#[component]
pub fn TripChip(trip: Trip, selected: bool, on_click: EventHandler<String>) -> Element {
    let trip_id = trip.id.clone();

    if selected {
        rsx! {
            button {
                class: "bg-primary text-white rounded-full px-3 py-1.5 text-sm flex items-center gap-1",
                onclick: move |_| on_click.call(trip_id.clone()),
                Icon { icon: LdCheck, width: 12, height: 12 }
                "{trip.name}"
            }
        }
    } else {
        rsx! {
            button {
                class: "border border-border text-foreground rounded-full px-3 py-1.5 text-sm bg-card",
                onclick: move |_| on_click.call(trip_id.clone()),
                "{trip.name}"
            }
        }
    }
}
