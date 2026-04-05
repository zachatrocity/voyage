use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdShare2};
use dioxus_free_icons::Icon;

use crate::api::{self, ApiError};
use crate::components::bottom_sheet::BottomSheet;
use crate::components::email_detail_card::EmailDetailCard;
use crate::components::trip_chip::TripChip;
use crate::notification::{notify_error, notify_success};
use crate::types::Email;
use crate::{SELECTED_EMAIL, TRIPS};

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

    let email_resource = use_resource(move || {
        let email_id = selected_id();
        let _nonce = refresh_nonce();
        async move {
            let id = email_id.ok_or_else(|| ApiError::Network("No email selected".to_string()))?;
            let email = api::get_email(&id).await?;
            Ok::<Email, ApiError>(to_ui_email(&email))
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

    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col h-screen bg-background",
            // Header
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
                        EmailDetailCard { email: email.clone() }
                    },
                }
            }

            // Bottom sheet: tag action
            BottomSheet {
                h3 { class: "text-base font-bold text-foreground mb-1", "Add to Trip" }
                p { class: "text-sm text-muted mb-4", "Choose a trip label for this email" }

                // Trip chips
                div { class: "flex flex-wrap gap-2 mb-4",
                    for trip in TRIPS.read().iter() {
                        TripChip {
                            key: "{trip.id}",
                            trip: trip.clone(),
                            selected: selected_trip_id() == Some(trip.id.clone()),
                            on_click: move |id: String| selected_trip_id.set(Some(id)),
                        }
                    }
                    // New trip button
                    button { class: "border border-dashed border-cta text-cta rounded-full px-3 py-1.5 text-sm",
                        "+ New Trip"
                    }
                }

                // Confirm button
                button {
                    class: "w-full bg-cta text-white rounded-xl py-3 font-semibold text-sm",
                    onclick: on_confirm,
                    "Confirm Tag"
                }
            }
        }
    }
}
