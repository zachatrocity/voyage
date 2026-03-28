use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdSettings;
use dioxus_free_icons::Icon;

#[component]
pub fn AppHeader() -> Element {
    rsx! {
        div { class: "flex items-center justify-between px-4 py-3 bg-card border-b border-border",
            span { class: "text-xl font-bold text-primary", "Voyage ✈️" }
            div { class: "flex items-center gap-2",
                div { class: "w-8 h-8 rounded-full bg-primary/20" }
                Icon { icon: LdSettings, width: 20, height: 20, class: "text-muted" }
            }
        }
    }
}
