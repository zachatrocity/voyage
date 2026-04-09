use dioxus::prelude::*;

use crate::components::status_badge::StatusBadge;
use crate::types::{Category, ItineraryItem};

fn category_emoji(cat: &Category) -> &'static str {
    match cat {
        Category::Flight => "✈️",
        Category::Hotel => "🏨",
        Category::CarRental => "🚗",
        Category::Cruise => "🚢",
        Category::Activity => "🎡",
        Category::Other => "📧",
    }
}

#[component]
pub fn TimelineItem(item: ItineraryItem, show_date: bool) -> Element {
    let emoji = category_emoji(&item.category);

    rsx! {
        div { class: "flex gap-3 mb-4",
            // Left: timeline line + icon
            div { class: "flex flex-col items-center",
                if show_date {
                    span { class: "text-[11px] font-semibold text-muted mb-1 w-16 text-center leading-tight",
                        "{item.date}"
                    }
                } else {
                    div { class: "h-5 w-16" }
                }
                div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-lg z-10",
                    "{emoji}"
                }
                div { class: "flex-1 w-0.5 bg-border mt-1" }
            }

            // Right: card
            div { class: "flex-1 bg-card rounded-xl shadow-sm p-3 mb-1",
                div { class: "flex items-start justify-between gap-2",
                    div { class: "flex-1",
                        p { class: "text-sm font-semibold text-foreground", "{item.title}" }
                        p { class: "text-xs text-muted mt-0.5", "{item.detail}" }
                        if let Some(ref sub) = item.sub_detail {
                            p { class: "text-xs text-muted/70 mt-0.5", "{sub}" }
                        }
                    }
                    StatusBadge { status: item.status.clone() }
                }
            }
        }
    }
}
