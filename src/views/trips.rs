use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError, TripResponse};
use crate::notification::{notify_error, notify_success};
use crate::types::Trip;
use crate::{Route, SELECTED_TRIP, TRIPS};

fn build_trip_deep_link(origin: &str, trip_id: &str) -> String {
    format!(
        "{}/itinerary?trip_id={}",
        origin.trim_end_matches('/'),
        trip_id
    )
}

fn map_trip_response_to_trip(t: &TripResponse) -> Trip {
    Trip {
        id: t.id.clone(),
        name: t.name.clone(),
        date_range: t.date_range.clone(),
        email_count: t.email_count,
        confirmed_count: t.confirmed_count,
    }
}

fn reconcile_selected_trip(current_selected: Option<String>, trips: &[Trip]) -> Option<String> {
    if let Some(selected) = current_selected {
        if trips.iter().any(|t| t.id == selected) {
            return Some(selected);
        }
    }

    trips.first().map(|t| t.id.clone())
}

async fn refresh_global_trips() -> Result<Vec<Trip>, ApiError> {
    let fresh = api::list_trips().await?;
    let mapped = fresh
        .trips
        .iter()
        .map(map_trip_response_to_trip)
        .collect::<Vec<_>>();

    *TRIPS.write() = mapped.clone();
    Ok(mapped)
}

#[component]
pub fn Trips() -> Element {
    let navigator = use_navigator();
    let mut refresh_nonce = use_signal(|| 0u64);
    let mut delete_in_flight = use_signal(|| Option::<String>::None);

    let trips_resource = use_resource(move || {
        let _nonce = refresh_nonce();
        async move { api::list_trips().await }
    });

    let trips = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Ok(resp)) => resp
            .trips
            .iter()
            .map(map_trip_response_to_trip)
            .collect::<Vec<_>>(),
        _ => Vec::new(),
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
                    *SELECTED_TRIP.write() = Some(new_trip.id.clone());

                    if let Err(err) = refresh_global_trips().await {
                        notify_error(format!(
                            "Trip created, but failed to sync refreshed trips: {err}"
                        ));
                    }

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
                        div {
                            key: "{trip.id}",
                            class: "rounded-xl bg-card shadow-sm p-4 mb-3",

                            button {
                                class: "w-full text-left",
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

                            div { class: "mt-3 pt-3 border-t border-border flex gap-2",
                                button {
                                    class: "text-xs px-3 py-1 rounded-full border border-border text-foreground",
                                    onclick: {
                                        let trip_id = trip.id.clone();
                                        move |_| {
                                            *SELECTED_TRIP.write() = Some(trip_id.clone());
                                            navigator.push(Route::Itinerary {});
                                        }
                                    },
                                    "Open"
                                }

                                button {
                                    class: "text-xs px-3 py-1 rounded-full border border-border text-foreground",
                                    onclick: {
                                        let trip_id = trip.id.clone();
                                        move |_| {
                                            let trip_id_for_share = trip_id.clone();
                                            spawn(async move {
                                                let mut origin_eval = document::eval("dioxus.send(window.location.origin || '');");
                                                let origin = origin_eval.recv::<String>().await.unwrap_or_default();
                                                let deep_link = build_trip_deep_link(&origin, &trip_id_for_share);
                                                let link_js = serde_json::to_string(&deep_link)
                                                    .unwrap_or_else(|_| "\"\"".to_string());

                                                let mut eval = document::eval(&format!(
                                                    r#"
                                                    const link = {link_js};
                                                    try {{
                                                        if (navigator.clipboard?.writeText) {{
                                                            await navigator.clipboard.writeText(link);
                                                            dioxus.send("copied");
                                                        }} else {{
                                                            dioxus.send("unsupported");
                                                        }}
                                                    }} catch (e) {{
                                                        dioxus.send("error");
                                                    }}
                                                    "#
                                                ));

                                                match eval.recv::<String>().await.unwrap_or_default().as_str() {
                                                    "copied" => notify_success("Trip deep link copied"),
                                                    "unsupported" => notify_error("Clipboard is not supported on this device"),
                                                    _ => notify_error("Failed to copy deep link"),
                                                }
                                            });
                                        }
                                    },
                                    "Share"
                                }

                                button {
                                    class: "text-xs px-3 py-1 rounded-full border border-red-300 text-red-600",
                                    disabled: delete_in_flight().as_ref() == Some(&trip.id),
                                    onclick: {
                                        let trip = trip.clone();
                                        move |_| {
                                            let trip_for_delete = trip.clone();
                                            if delete_in_flight().as_ref() == Some(&trip_for_delete.id) {
                                                return;
                                            }

                                            delete_in_flight.set(Some(trip_for_delete.id.clone()));
                                            spawn(async move {
                                                let confirm_text = format!(
                                                    "Delete trip '{}' ? This cannot be undone.",
                                                    trip_for_delete.name
                                                );
                                                let confirm_js = serde_json::to_string(&confirm_text)
                                                    .unwrap_or_else(|_| "\"Delete this trip?\"".to_string());
                                                let mut eval = document::eval(&format!(
                                                    "dioxus.send(String(window.confirm({confirm_js})));"
                                                ));
                                                let confirmed = eval
                                                    .recv::<String>()
                                                    .await
                                                    .unwrap_or_default()
                                                    == "true";

                                                if !confirmed {
                                                    delete_in_flight.set(None);
                                                    return;
                                                }

                                                match api::delete_trip(&trip_for_delete.id).await {
                                                    Ok(_) => {
                                                        let previous_selection = SELECTED_TRIP.read().clone();
                                                        match refresh_global_trips().await {
                                                            Ok(fresh_trips) => {
                                                                *SELECTED_TRIP.write() = reconcile_selected_trip(
                                                                    previous_selection,
                                                                    &fresh_trips,
                                                                );
                                                            }
                                                            Err(err) => {
                                                                notify_error(format!(
                                                                    "Trip deleted, but failed to sync refreshed trips: {err}"
                                                                ));
                                                            }
                                                        }

                                                        refresh_nonce += 1;
                                                        notify_success("Trip deleted");
                                                    }
                                                    Err(ApiError::Server { status: 404, .. }) => {
                                                        notify_error("Delete is not available on this backend yet");
                                                    }
                                                    Err(err) => {
                                                        notify_error(format!("Delete failed: {err}"));
                                                    }
                                                }

                                                delete_in_flight.set(None);
                                            });
                                        }
                                    },
                                    if delete_in_flight().as_ref() == Some(&trip.id) {
                                        "Deleting..."
                                    } else {
                                        "Delete"
                                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_trip(id: &str) -> Trip {
        Trip {
            id: id.to_string(),
            name: format!("Trip {id}"),
            date_range: "Dates TBD".to_string(),
            email_count: 0,
            confirmed_count: 0,
        }
    }

    #[test]
    fn deep_link_format_is_expected() {
        let link = build_trip_deep_link("https://voyage.local", "trip-123");
        assert_eq!(link, "https://voyage.local/itinerary?trip_id=trip-123");
    }

    #[test]
    fn reconcile_keeps_existing_selection_when_present() {
        let trips = vec![mk_trip("a"), mk_trip("b")];
        let selected = reconcile_selected_trip(Some("b".to_string()), &trips);
        assert_eq!(selected, Some("b".to_string()));
    }

    #[test]
    fn reconcile_falls_back_to_first_trip_when_selection_is_missing() {
        let trips = vec![mk_trip("a"), mk_trip("b")];
        let selected = reconcile_selected_trip(Some("missing".to_string()), &trips);
        assert_eq!(selected, Some("a".to_string()));
    }

    #[test]
    fn reconcile_returns_none_when_no_trips_exist() {
        let trips = vec![];
        let selected = reconcile_selected_trip(Some("missing".to_string()), &trips);
        assert_eq!(selected, None);
    }
}
