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
                div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-lg",
                    "{emoji}"
                }
                div { class: "flex-1",
                    p { class: "text-sm font-semibold text-foreground", "{email.sender}" }
                    p { class: "text-xs text-muted", "{email.sender_email}" }
                }
                span { class: "text-xs text-muted", "{email.date}" }
            }

            // Subject
            h2 { class: "text-base font-bold text-foreground mb-2", "{email.subject}" }

            // Category badge
            span { class: "text-xs border border-primary/30 text-primary rounded-full px-2 py-0.5 inline-block mb-3",
                "{email.category.label()}"
            }

            // Body preview with fade
            div { class: "relative",
                p { class: "text-sm text-muted leading-relaxed line-clamp-4", "{email.body_preview}" }
                div { class: "absolute bottom-0 left-0 right-0 h-8 bg-gradient-to-t from-card to-transparent" }
            }
        }
    }
}
