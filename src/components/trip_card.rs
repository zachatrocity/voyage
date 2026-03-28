use crate::types::Trip;
use dioxus::prelude::*;

fn trip_emoji(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    if lower.contains("disney") || lower.contains("theme park") {
        "🎡"
    } else if lower.contains("cruise") || lower.contains("caribbean") {
        "🚢"
    } else if lower.contains("beach") || lower.contains("island") {
        "🏝️"
    } else if lower.contains("ski") || lower.contains("mountain") {
        "🏔️"
    } else {
        "✈️"
    }
}

#[component]
pub fn TripCard(trip: Trip) -> Element {
    let emoji = trip_emoji(&trip.name);
    let pct = if trip.email_count > 0 {
        (trip.confirmed_count as f64 / trip.email_count as f64) * 100.0
    } else {
        0.0
    };
    let pct_str = format!("{pct}%");

    rsx! {
        div { class: "rounded-xl bg-card shadow-sm p-4 mb-3 flex gap-3",
            div { class: "w-16 h-16 rounded-lg bg-primary/10 flex items-center justify-center text-2xl shrink-0",
                "{emoji}"
            }
            div { class: "flex flex-col flex-1 min-w-0",
                span { class: "font-semibold text-foreground", "{trip.name}" }
                span { class: "text-sm text-muted", "{trip.date_range}" }
                div { class: "flex gap-2 mt-1",
                    span { class: "text-xs px-2 py-0.5 rounded-full border border-primary text-primary",
                        "{trip.email_count} emails"
                    }
                    span { class: "text-xs px-2 py-0.5 rounded-full border border-primary text-primary",
                        "{trip.confirmed_count} confirmed"
                    }
                }
                div { class: "w-full h-1.5 rounded-full bg-border mt-2",
                    div { class: "h-1.5 rounded-full bg-primary", width: "{pct_str}" }
                }
            }
        }
    }
}
