use dioxus::prelude::*;

use crate::types::{Category, Email};
use crate::TRIPS;

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
pub fn EmailListItem(email: Email, on_click: EventHandler<String>) -> Element {
    let id = email.id.clone();
    let tags_line = if email.tags.is_empty() {
        "No tags".to_string()
    } else {
        email
            .tags
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(" · ")
    };
    let trip_label = email.trip_id.as_ref().and_then(|tid| {
        TRIPS
            .read()
            .iter()
            .find(|t| &t.id == tid)
            .map(|t| t.name.clone())
    });

    rsx! {
        div {
            class: "mx-4 mb-2 bg-card rounded-xl shadow-sm px-4 py-3 flex items-center gap-3 cursor-pointer",
            onclick: move |_| on_click.call(id.clone()),

            // Left: category emoji circle
            div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center shrink-0 text-lg",
                "{category_emoji(&email.category)}"
            }

            // Middle: subject + tags + sender/date
            div { class: "flex-1 min-w-0",
                div { class: "text-sm font-semibold text-foreground truncate",
                    "{email.subject}"
                }
                div { class: "text-xs text-muted mt-0.5 truncate",
                    "{tags_line}"
                }
                div { class: "flex justify-between items-center mt-0.5",
                    span { class: "text-xs text-muted truncate",
                        "{email.sender}"
                    }
                    span { class: "text-xs text-muted shrink-0 ml-2",
                        "{email.date}"
                    }
                }
            }

            // Right: category badge + trip label
            div { class: "flex flex-col items-end gap-1 shrink-0",
                span { class: "text-xs border rounded-full px-2 py-0.5 {email.category.color_class()} border-current",
                    "{email.category.label()}"
                }
                if let Some(label) = &trip_label {
                    span { class: "bg-primary/10 text-primary text-xs rounded-full px-2 py-0.5",
                        "{label}"
                    }
                } else {
                    span { class: "border border-border text-muted text-xs rounded-full px-2 py-0.5",
                        "Unlabeled"
                    }
                }
            }
        }
    }
}
