use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdSearch;
use dioxus_free_icons::Icon;

#[component]
pub fn SearchBar(value: String, on_change: EventHandler<String>) -> Element {
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
        }
    }
}
