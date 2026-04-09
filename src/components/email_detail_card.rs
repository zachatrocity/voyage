use dioxus::prelude::*;

use crate::types::{Category, Email};

#[component]
pub fn EmailDetailCard(email: Email) -> Element {
    let emoji = match email.category {
        Category::Flight => "✈️",
        Category::Hotel => "🏨",
        Category::CarRental => "🚗",
        Category::Cruise => "🚢",
        Category::Activity => "🎡",
        Category::Other => "📧",
    };

    rsx! {
        div { class: "bg-card rounded-xl shadow-sm mx-4 mt-4 p-4",
            // Sender row
            div { class: "flex items-center gap-3 mb-3",
                div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-lg shrink-0",
                    "{emoji}"
                }
                div { class: "flex-1 min-w-0",
                    p { class: "text-sm font-semibold text-foreground truncate", "{email.sender}" }
                    p { class: "text-xs text-muted truncate", "{email.sender_email}" }
                }
                span { class: "text-xs text-muted shrink-0", "{email.date}" }
            }

            // Subject
            h2 { class: "text-base font-bold text-foreground mb-2", "{email.subject}" }

            // Category badge
            span { class: "text-xs border border-primary/30 text-primary rounded-full px-2 py-0.5 inline-block mb-3",
                "{email.category.label()}"
            }

            // Body
            div { class: "max-h-[52vh] overflow-y-auto pr-1",
                p { class: "text-sm text-muted leading-relaxed whitespace-pre-wrap break-words", "{email.body_preview}" }
            }
        }
    }
}
