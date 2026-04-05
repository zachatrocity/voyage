use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::notification::{notify_error, notify_success};
use crate::types::Trip;
use crate::{Route, SELECTED_TRIP, TRIPS};

#[component]
pub fn Trips() -> Element {
    let navigator = use_navigator();
    let mut refresh_nonce = use_signal(|| 0u64);

    let trips_resource = use_resource(move || {
        let _nonce = refresh_nonce();
        async move { api::list_trips().await }
    });

    let trips = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Ok(resp)) => resp
            .trips
            .iter()
            .map(|t| Trip {
                id: t.id.clone(),
                name: t.name.clone(),
                date_range: t.date_range.clone(),
                email_count: t.email_count,
                confirmed_count: t.confirmed_count,
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    });

    use_effect(move || {
        if !trips().is_empty() {
            *TRIPS.write() = trips();
        }
    });

    let load_error = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Err(err)) => Some(match err {
            ApiError::Network(msg) => format!("Network error: {msg}"),
            ApiError::Decode(msg) => format!("Decode error: {msg}"),
            ApiError::Server { status, message } => format!("Server error ({status}): {message}"),
        }),
        _ => None,
    });

    let on_add_trip = move |_| {
        let fallback_name = match &*trips_resource.read_unchecked() {
            Some(Ok(resp)) => format!("New Trip {}", resp.trips.len() + 1),
            _ => "New Trip".to_string(),
        };

        spawn(async move {
            let mut eval = document::eval(
                r#"
                const input = window.prompt("Trip name", "");
                dioxus.send(input ?? "");
                "#,
            );

            let entered = eval.recv::<String>().await.unwrap_or_default();
            let trip_name = if entered.trim().is_empty() {
                fallback_name
            } else {
                entered.trim().to_string()
            };

            match api::create_trip(&trip_name, "Dates TBD").await {
                Ok(new_trip) => {
                    *SELECTED_TRIP.write() = Some(new_trip.id);
                    refresh_nonce += 1;
                    notify_success("Trip created");
                    navigator.push(Route::Itinerary {});
                }
                Err(err) => notify_error(format!("Failed to create trip: {err}")),
            }
        });
    };

    rsx! {
        div { class: "flex flex-col h-full bg-background",
            div { class: "bg-card border-b border-border px-4 py-3",
                h1 { class: "text-lg font-semibold text-foreground", "Trips" }
            }

            div { class: "flex-1 overflow-y-auto px-4 pt-4 pb-24",
                if trips_resource.read_unchecked().is_none() {
                    for i in 0..3 {
                        div {
                            key: "trip-skeleton-{i}",
                            class: "mb-3 rounded-xl border border-border bg-card p-4 animate-pulse",
                            div { class: "h-5 w-1/2 bg-border rounded mb-2" }
                            div { class: "h-4 w-1/3 bg-border rounded" }
                        }
                    }
                } else if let Some(err) = load_error() {
                    div { class: "rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700",
                        "{err}"
                    }
                } else if trips().is_empty() {
                    div { class: "flex flex-col items-center justify-center py-12 text-muted",
                        span { class: "text-4xl mb-2", "🧳" }
                        span { class: "text-sm", "No trips yet" }
                    }
                } else {
                    for trip in trips().iter() {
                        button {
                            key: "{trip.id}",
                            class: "w-full text-left rounded-xl bg-card shadow-sm p-4 mb-3",
                            onclick: {
                                let trip_id = trip.id.clone();
                                move |_| {
                                    *SELECTED_TRIP.write() = Some(trip_id.clone());
                                    navigator.push(Route::Itinerary {});
                                }
                            },
                            div { class: "font-semibold text-foreground", "{trip.name}" }
                            div { class: "text-sm text-muted", "{trip.date_range}" }
                            div { class: "flex gap-2 mt-2",
                                span { class: "text-xs px-2 py-0.5 rounded-full border border-primary text-primary",
                                    "{trip.email_count} emails"
                                }
                                span { class: "text-xs px-2 py-0.5 rounded-full border border-primary text-primary",
                                    "{trip.confirmed_count} confirmed"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "fixed bottom-20 right-4",
                button {
                    class: "w-14 h-14 rounded-full bg-cta shadow-lg flex items-center justify-center",
                    onclick: on_add_trip,
                    Icon { icon: LdPlus, width: 24, height: 24, class: "text-white" }
                }
            }
        }
    }
}
