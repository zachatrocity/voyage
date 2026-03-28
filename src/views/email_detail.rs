use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdShare2};
use dioxus_free_icons::Icon;

use crate::components::bottom_sheet::BottomSheet;
use crate::components::email_detail_card::EmailDetailCard;
use crate::components::trip_chip::TripChip;
use crate::{EMAILS, SELECTED_EMAIL, TRIPS};

#[component]
pub fn EmailDetail() -> Element {
    let email = use_memo(move || {
        let emails = EMAILS.read();
        let selected_id = SELECTED_EMAIL.read();
        selected_id
            .as_ref()
            .and_then(|id| emails.iter().find(|e| &e.id == id).cloned())
    });

    let mut selected_trip_id = use_signal(|| Option::<String>::None);

    let on_confirm = move |_| {
        if let Some(trip_id) = selected_trip_id() {
            let mut emails = EMAILS.write();
            if let Some(selected_id) = SELECTED_EMAIL.read().as_ref() {
                if let Some(email) = emails.iter_mut().find(|e| &e.id == selected_id) {
                    email.trip_id = Some(trip_id.clone());
                }
            }
            // Update trip email_count
            let mut trips = TRIPS.write();
            if let Some(trip) = trips.iter_mut().find(|t| t.id == trip_id) {
                trip.email_count += 1;
            }
        }
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
                    {email().as_ref().map(|e| e.subject.clone()).unwrap_or_default()}
                }
                Icon { icon: LdShare2, width: 20, height: 20, class: "text-muted" }
            }

            // Email detail card
            if let Some(ref e) = email() {
                EmailDetailCard { email: e.clone() }
            }

            // Spacer to push bottom sheet content area
            div { class: "flex-1" }

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
