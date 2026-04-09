use crate::api::{self, ApiError};
use crate::components::app_header::AppHeader;
use crate::components::suggested_email_card::SuggestedEmailCard;
use crate::components::trip_card::TripCard;
use crate::types::Trip;
use crate::EMAILS;
use dioxus::prelude::*;

fn map_api_trip(t: &api::TripResponse) -> Trip {
    Trip {
        id: t.id.clone(),
        name: t.name.clone(),
        date_range: t.date_range.clone(),
        email_count: t.email_count,
        confirmed_count: t.confirmed_count,
    }
}

#[component]
pub fn Home() -> Element {
    let emails = EMAILS.read();
    let untagged: Vec<_> = emails.iter().filter(|e| e.trip_id.is_none()).cloned().collect();

    let trips_resource = use_resource(move || async move { api::list_trips().await });

    let trips = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Ok(resp)) => resp.trips.iter().map(map_api_trip).collect::<Vec<_>>(),
        _ => Vec::new(),
    });

    let trip_error = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Err(err)) => Some(match err {
            ApiError::Network(msg) => format!("Network error: {msg}"),
            ApiError::Decode(msg) => format!("Decode error: {msg}"),
            ApiError::Server { status, message } => format!("Server error ({status}): {message}"),
        }),
        _ => None,
    });

    rsx! {
        div { class: "flex flex-col h-full bg-background",
            AppHeader {}
            div { class: "overflow-y-auto flex-1",
                div { class: "px-4 pt-4 pb-4",
                    h2 { class: "text-lg font-semibold text-foreground mb-3", "Upcoming Trips" }

                    if trips_resource.read_unchecked().is_none() {
                        for i in 0..2 {
                            div {
                                key: "home-trip-skeleton-{i}",
                                class: "rounded-xl bg-card shadow-sm p-4 mb-3 animate-pulse",
                                div { class: "h-5 w-1/2 bg-border rounded mb-2" }
                                div { class: "h-4 w-1/3 bg-border rounded" }
                            }
                        }
                    } else if let Some(err) = trip_error() {
                        div { class: "rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700 mb-3",
                            "{err}"
                        }
                    } else if trips().is_empty() {
                        div { class: "rounded-xl border border-border bg-card px-4 py-6 text-sm text-muted text-center mb-3",
                            "No trips yet"
                        }
                    } else {
                        for trip in trips().iter() {
                            TripCard { key: "{trip.id}", trip: trip.clone() }
                        }
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
