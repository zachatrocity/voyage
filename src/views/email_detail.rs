use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdShare2};
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::components::bottom_sheet::BottomSheet;
use crate::components::email_detail_card::EmailDetailCard;
use crate::components::trip_chip::TripChip;
use crate::notification::{notify_error, notify_success};
use crate::trip_creation::prompt_trip_creation;
use crate::types::{Email, Trip};
use crate::SELECTED_EMAIL;

fn to_ui_email(e: &api::EmailResult) -> Email {
    Email {
        id: e.id.clone(),
        subject: e.subject.clone(),
        sender: e.sender.clone(),
        sender_email: e.sender_email.clone(),
        date: e.date.clone(),
        body_preview: e.body_preview.clone(),
        category: e.category.clone(),
        tags: e.tags.clone(),
        trip_id: e.trip_id.clone(),
    }
}

#[component]
pub fn EmailDetail() -> Element {
    let selected_id = use_memo(move || SELECTED_EMAIL.read().clone());
    let mut selected_trip_id = use_signal(|| Option::<String>::None);
    let mut refresh_nonce = use_signal(|| 0u64);
    let mut trips_refresh_nonce = use_signal(|| 0u64);

    let email_resource = use_resource(move || {
        let email_id = selected_id();
        let _nonce = refresh_nonce();
        async move {
            let id = email_id.ok_or_else(|| ApiError::Network("No email selected".to_string()))?;
            let email = api::get_email(&id).await?;
            Ok::<Email, ApiError>(to_ui_email(&email))
        }
    });

    let trips_resource = use_resource(move || {
        let _nonce = trips_refresh_nonce();
        async move { api::list_trips().await }
    });

    let email_content_resource = use_resource(move || {
        let email_id = selected_id();
        let _nonce = refresh_nonce();
        async move {
            match email_id {
                Some(id) => api::get_email_content(&id).await,
                None => Ok(api::EmailContentResponse {
                    message_id: String::new(),
                    body: String::new(),
                    html_body: None,
                }),
            }
        }
    });

    let on_confirm = move |_| {
        let trip_id = selected_trip_id();
        let email_id = selected_id();

        spawn(async move {
            let Some(trip_id) = trip_id else {
                notify_error("Choose a trip first");
                return;
            };
            let Some(email_id) = email_id else {
                notify_error("No email selected");
                return;
            };

            let tag = format!("trip:{trip_id}");
            match api::tag_email(&email_id, &tag).await {
                Ok(_) => {
                    notify_success("Tagged email successfully");
                    refresh_nonce += 1;
                }
                Err(err) => notify_error(format!("Tagging failed: {err}")),
            }
        });
    };

    let on_new_trip = move |_| {
        let fallback_name = match &*trips_resource.read_unchecked() {
            Some(Ok(resp)) => format!("New Trip {}", resp.trips.len() + 1),
            _ => "New Trip".to_string(),
        };

        spawn(async move {
            let Some(input) = prompt_trip_creation(&fallback_name).await else {
                return;
            };

            match api::create_trip(&input.name, &input.date_range).await {
                Ok(created) => {
                    selected_trip_id.set(Some(created.id));
                    trips_refresh_nonce += 1;
                    notify_success("Trip created");
                }
                Err(err) => notify_error(format!("Failed to create trip: {err}")),
            }
        });
    };

    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col h-screen bg-background",
            div { class: "bg-card border-b border-border px-4 py-3 flex items-center gap-3",
                button {
                    class: "text-foreground",
                    onclick: move |_| { navigator.go_back(); },
                    Icon { icon: LdArrowLeft, width: 20, height: 20 }
                }
                span { class: "flex-1 text-sm font-medium text-foreground truncate",
                    match &*email_resource.read_unchecked() {
                        Some(Ok(email)) => email.subject.clone(),
                        _ => "Email detail".to_string(),
                    }
                }
                Icon { icon: LdShare2, width: 20, height: 20, class: "text-muted" }
            }

            div { class: "flex-1 overflow-y-auto pb-56",
                match &*email_resource.read_unchecked() {
                    None => rsx! {
                        div { class: "mx-4 mt-4 rounded-xl border border-border bg-card p-4 animate-pulse",
                            div { class: "h-4 w-2/3 bg-border rounded mb-3" }
                            div { class: "h-3 w-1/2 bg-border rounded mb-2" }
                            div { class: "h-3 w-full bg-border rounded" }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { class: "mx-4 mt-4 rounded-lg border border-red-300 bg-red-50 px-3 py-2 text-sm text-red-700",
                            "{err}"
                        }
                    },
                    Some(Ok(email)) => rsx! {
                        EmailDetailCard {
                            email: email.clone(),
                            full_body: match &*email_content_resource.read_unchecked() {
                                Some(Ok(content)) if !content.body.trim().is_empty() => Some(content.body.clone()),
                                _ => None,
                            },
                            full_html: match &*email_content_resource.read_unchecked() {
                                Some(Ok(content)) => content.html_body.clone(),
                                _ => None,
                            },
                            loading_full_body: email_content_resource.read_unchecked().is_none(),
                        }
                    },
                }
            }

            BottomSheet {
                h3 { class: "text-base font-bold text-foreground mb-1", "Add to Trip" }
                p { class: "text-sm text-muted mb-4", "Choose a trip label for this email" }

                div { class: "flex flex-wrap gap-2 mb-4",
                    match &*trips_resource.read_unchecked() {
                        None => rsx! { span { class: "text-xs text-muted", "Loading trips..." } },
                        Some(Err(err)) => rsx! { span { class: "text-xs text-red-600", "{err}" } },
                        Some(Ok(resp)) => rsx! {
                            for trip in resp.trips.iter() {
                                TripChip {
                                    key: "{trip.id}",
                                    trip: Trip {
                                        id: trip.id.clone(),
                                        name: trip.name.clone(),
                                        date_range: trip.date_range.clone(),
                                        email_count: trip.email_count,
                                        confirmed_count: trip.confirmed_count,
                                    },
                                    selected: selected_trip_id() == Some(trip.id.clone()),
                                    on_click: move |id: String| selected_trip_id.set(Some(id)),
                                }
                            }
                        },
                    }

                    button {
                        class: "border border-dashed border-cta text-cta rounded-full px-3 py-1.5 text-sm",
                        onclick: on_new_trip,
                        "+ New Trip"
                    }
                }

                button {
                    class: "w-full bg-cta text-white rounded-xl py-3 font-semibold text-sm",
                    onclick: on_confirm,
                    "Confirm Tag"
                }
            }
        }
    }
}
