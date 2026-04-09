use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlus;
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::components::hero_header::HeroHeader;
use crate::components::timeline_item::TimelineItem;
use crate::notification::{notify_error, notify_success};
use crate::types::{Category, ItineraryItem, ItineraryStatus, Trip};
use crate::{SELECTED_TRIP, TRIPS};

fn diag(msg: impl AsRef<str>) {
    eprintln!("[diag][itinerary] {}", msg.as_ref());
}

fn infer_category(subject: &str, tags: &[String]) -> Category {
    let s = subject.to_ascii_lowercase();
    let has_tag = |needle: &str| tags.iter().any(|t| t.to_ascii_lowercase().contains(needle));

    if s.contains("flight") || s.contains("airlines") || s.contains("ticket") || has_tag("flight") {
        Category::Flight
    } else if s.contains("hotel") || s.contains("reservation") || has_tag("hotel") {
        Category::Hotel
    } else if s.contains("rental") || s.contains("hertz") || s.contains("avis") || has_tag("car") {
        Category::CarRental
    } else if s.contains("cruise") || has_tag("cruise") {
        Category::Cruise
    } else if s.contains("park") || s.contains("activity") || has_tag("activity") {
        Category::Activity
    } else {
        Category::Other
    }
}

fn format_timeline_date(raw: &str) -> String {
    let date_part = raw.split('T').next().unwrap_or(raw).trim();
    let mut parts = date_part.split('-');
    let (year, month, day) = match (parts.next(), parts.next(), parts.next()) {
        (Some(y), Some(m), Some(d)) => (y, m, d),
        _ => return date_part.to_string(),
    };

    let month_name = match month {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return date_part.to_string(),
    };

    let day = day.trim_start_matches('0');
    let day = if day.is_empty() { "0" } else { day };
    format!("{month_name} {day}, {year}")
}

fn map_trip_email_to_timeline(trip_id: &str, e: &api::TripEmailItem) -> ItineraryItem {
    let title = if e.subject.trim().is_empty() {
        "Untitled email".to_string()
    } else {
        e.subject.clone()
    };

    ItineraryItem {
        id: e.id.clone(),
        trip_id: trip_id.to_string(),
        email_id: e.id.clone(),
        title,
        detail: e.sender.clone(),
        sub_detail: None,
        date: format_timeline_date(&e.date),
        category: infer_category(&e.subject, &e.tags),
        status: ItineraryStatus::Confirmed,
    }
}

#[component]
pub fn Itinerary() -> Element {
    let mut refresh_nonce = use_signal(|| 0u64);

    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                const id = new URLSearchParams(window.location.search).get("trip_id") || "";
                dioxus.send(id);
                "#,
            );
            let trip_id = eval.recv::<String>().await.unwrap_or_default();
            if !trip_id.trim().is_empty()
                && SELECTED_TRIP.read().as_deref() != Some(trip_id.as_str())
            {
                diag(format!(
                    "SELECTED_TRIP write (query param sync): {}",
                    trip_id
                ));
                *SELECTED_TRIP.write() = Some(trip_id);
            }
        });
    });

    let trips_resource = use_resource(move || {
        let _nonce = refresh_nonce();
        async move { api::list_trips().await }
    });

    let selected_trip_id = use_memo(move || {
        let selected = SELECTED_TRIP.read().clone();
        match &*trips_resource.read_unchecked() {
            Some(Ok(resp)) => {
                if let Some(id) = selected {
                    if resp.trips.iter().any(|t| t.id == id) {
                        return Some(id);
                    }
                }
                resp.trips.first().map(|t| t.id.clone())
            }
            _ => selected,
        }
    });

    use_effect(move || {
        if let Some(id) = selected_trip_id() {
            if SELECTED_TRIP.read().as_ref() != Some(&id) {
                diag(format!(
                    "SELECTED_TRIP write (selected_trip_id effect): {}",
                    id
                ));
                *SELECTED_TRIP.write() = Some(id);
            }
        }
    });

    let trip: Memo<Option<Trip>> = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Ok(resp)) => {
            let id = selected_trip_id();
            id.and_then(|sid| resp.trips.iter().find(|t| t.id == sid).cloned())
                .or_else(|| resp.trips.first().cloned())
                .map(|t| Trip {
                    id: t.id,
                    name: t.name,
                    date_range: t.date_range,
                    email_count: t.email_count,
                    confirmed_count: t.confirmed_count,
                })
        }
        _ => None,
    });

    let trip_emails_resource = use_resource(move || {
        let trip_id = selected_trip_id();
        let _nonce = refresh_nonce();
        async move {
            match trip_id {
                Some(id) => api::get_trip_emails(&id).await.map(|resp| resp.emails),
                None => Ok(vec![]),
            }
        }
    });

    let items = use_memo(move || {
        let trip_id = selected_trip_id();
        match (trip_id, &*trip_emails_resource.read_unchecked()) {
            (Some(id), Some(Ok(emails))) => emails
                .iter()
                .map(|e| map_trip_email_to_timeline(&id, e))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        }
    });

    let trip_error = use_memo(move || match &*trips_resource.read_unchecked() {
        Some(Err(err)) => Some(match err {
            ApiError::Network(msg) => format!("Network error: {msg}"),
            ApiError::Decode(msg) => format!("Decode error: {msg}"),
            ApiError::Server { status, message } => format!("Server error ({status}): {message}"),
        }),
        _ => None,
    });

    let timeline_error = use_memo(move || match &*trip_emails_resource.read_unchecked() {
        Some(Err(err)) => Some(match err {
            ApiError::Network(msg) => format!("Network error: {msg}"),
            ApiError::Decode(msg) => format!("Decode error: {msg}"),
            ApiError::Server { status, message } => format!("Server error ({status}): {message}"),
        }),
        _ => None,
    });

    let on_add_trip = move |_| {
        diag("add_trip: trigger");
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
            diag("add_trip: prompt resolved");
            let trip_name = if entered.trim().is_empty() {
                fallback_name
            } else {
                entered.trim().to_string()
            };

            diag("add_trip: create_trip request");
            match api::create_trip(&trip_name, "Dates TBD").await {
                Ok(new_trip) => {
                    diag(format!(
                        "add_trip: create_trip success trip_id={}",
                        new_trip.id
                    ));
                    diag(format!(
                        "SELECTED_TRIP write (add_trip create success): {}",
                        new_trip.id
                    ));
                    *SELECTED_TRIP.write() = Some(new_trip.id.clone());

                    diag("add_trip: list_trips refresh start");
                    match api::list_trips().await {
                        Ok(fresh) => {
                            let mapped_trips = fresh
                                .trips
                                .into_iter()
                                .map(|t| Trip {
                                    id: t.id,
                                    name: t.name,
                                    date_range: t.date_range,
                                    email_count: t.email_count,
                                    confirmed_count: t.confirmed_count,
                                })
                                .collect::<Vec<_>>();
                            diag(format!(
                                "TRIPS write (post-add refresh): {} trips",
                                mapped_trips.len()
                            ));
                            *TRIPS.write() = mapped_trips;
                        }
                        Err(err) => {
                            diag(format!("add_trip: list_trips refresh failed error={err}"));
                        }
                    }

                    refresh_nonce += 1;
                    diag(format!(
                        "refresh_nonce incremented -> {} (add_trip)",
                        refresh_nonce()
                    ));
                    notify_success("Trip created");
                }
                Err(err) => {
                    diag(format!("add_trip: create_trip failed error={err}"));
                    notify_error(format!("Failed to create trip: {err}"))
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col h-full bg-background",
            if let Some(err) = trip_error() {
                div { class: "mx-4 mt-4 rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700",
                    "{err}"
                }
            } else if let Some(current_trip) = trip() {
                HeroHeader { trip: current_trip }

                div { class: "flex-1 overflow-y-auto px-4 pt-4 pb-24",
                    if trip_emails_resource.read_unchecked().is_none() {
                        for i in 0..4 {
                            div {
                                key: "timeline-skeleton-{i}",
                                class: "mb-3 rounded-xl border border-border bg-card p-4 animate-pulse",
                                div { class: "h-4 w-2/3 bg-border rounded mb-2" }
                                div { class: "h-3 w-1/2 bg-border rounded" }
                            }
                        }
                    } else if let Some(err) = timeline_error() {
                        div { class: "rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700",
                            "{err}"
                        }
                    } else if items().is_empty() {
                        div { class: "flex flex-col items-center justify-center py-12 text-muted",
                            span { class: "text-4xl mb-2", "🧳" }
                            span { class: "text-sm", "No tagged emails for this trip yet" }
                        }
                    } else {
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
                }

            } else if trips_resource.read_unchecked().is_none() {
                div { class: "mx-4 mt-4 rounded-xl border border-border bg-card p-4 animate-pulse",
                    div { class: "h-5 w-1/2 bg-border rounded mb-3" }
                    div { class: "h-4 w-2/3 bg-border rounded" }
                }
            } else {
                div { class: "flex flex-col items-center justify-center py-12 text-muted",
                    span { class: "text-4xl mb-2", "🗺️" }
                    span { class: "text-sm", "No trips found yet" }
                }
            }

            // FAB
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
