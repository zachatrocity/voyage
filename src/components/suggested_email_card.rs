use crate::types::{Category, Email};
use dioxus::prelude::*;

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
pub fn SuggestedEmailCard(email: Email, on_tag: EventHandler<String>) -> Element {
    let emoji = category_emoji(&email.category);
    let id = email.id.clone();

    rsx! {
        div { class: "rounded-xl bg-card shadow-sm p-4 mb-2 flex items-center gap-3",
            div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center shrink-0",
                "{emoji}"
            }
            div { class: "flex flex-col flex-1 min-w-0",
                span { class: "text-sm font-semibold text-foreground truncate", "{email.subject}" }
                span { class: "text-xs text-muted", "{email.sender} · {email.date}" }
            }
            button {
                class: "bg-cta text-white text-xs px-3 py-1.5 rounded-full shrink-0 hover:opacity-90 transition-opacity",
                onclick: move |_| on_tag.call(id.clone()),
                "Tag it"
            }
        }
    }
}
