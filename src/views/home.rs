use crate::components::app_header::AppHeader;
use crate::components::suggested_email_card::SuggestedEmailCard;
use crate::components::trip_card::TripCard;
use crate::{EMAILS, TRIPS};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let trips = TRIPS.read();
    let emails = EMAILS.read();
    let untagged: Vec<_> = emails.iter().filter(|e| e.trip_id.is_none()).cloned().collect();

    rsx! {
        div { class: "flex flex-col h-full bg-background",
            AppHeader {}
            div { class: "overflow-y-auto flex-1",
                div { class: "px-4 pt-4 pb-4",
                    h2 { class: "text-lg font-semibold text-foreground mb-3", "Upcoming Trips" }
                    for trip in trips.iter() {
                        TripCard { key: "{trip.id}", trip: trip.clone() }
                    }

                    div { class: "mt-6",
                        div { class: "flex items-center justify-between mb-3",
                            h2 { class: "text-lg font-semibold text-foreground", "Suggested Emails" }
                            span { class: "text-xs text-muted", "We think these are travel-related" }
                        }
                        for email in untagged.iter() {
                            SuggestedEmailCard {
                                key: "{email.id}",
                                email: email.clone(),
                                on_tag: move |_id: String| {
                                    // TODO: open tag/trip assignment flow
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
