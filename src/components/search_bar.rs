use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdSearch;
use dioxus_free_icons::Icon;

#[component]
pub fn SearchBar(value: String, on_change: EventHandler<String>) -> Element {
    let mut show_advanced_help = use_signal(|| false);

    rsx! {
        div { class: "px-4 pt-3 pb-2",
            div { class: "flex items-center gap-2 bg-card rounded-xl px-3 py-2.5 border border-border",
                Icon { icon: LdSearch, width: 16, height: 16, class: "text-muted shrink-0" }
                input {
                    class: "flex-1 bg-transparent text-sm text-foreground placeholder:text-muted outline-none",
                    placeholder: "Search travel emails...",
                    value: "{value}",
                    oninput: move |e| on_change.call(e.value()),
                }
            }

            div { class: "mt-2 px-1",
                button {
                    class: "text-xs text-muted underline underline-offset-2",
                    r#type: "button",
                    onclick: move |_| show_advanced_help.set(!show_advanced_help()),
                    if show_advanced_help() {
                        "Hide advanced search tips"
                    } else {
                        "Advanced search tips"
                    }
                }
            }

            if show_advanced_help() {
                div { class: "mt-2 rounded-lg border border-border bg-card px-3 py-2 text-xs text-muted",
                    p { class: "font-medium text-foreground mb-1", "Notmuch query examples" }
                    ul { class: "space-y-0.5 list-disc pl-4",
                        li { "from:delta" }
                        li { "subject:\"flight\"" }
                        li { "tag:unread" }
                        li { "date:2026-04-01..2026-04-30" }
                        li { "from:delta AND subject:itinerary" }
                    }
                }
            }
        }
    }
}
