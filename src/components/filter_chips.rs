use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterChip {
    pub key: String,
    pub label: String,
}

#[component]
pub fn FilterChips(
    active: String,
    filters: Vec<FilterChip>,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "flex gap-2 px-4 overflow-x-auto pb-2 scrollbar-hide",
            for filter in filters.into_iter() {
                {
                    let is_active = active == filter.key;
                    let class = if is_active {
                        "bg-primary text-white rounded-full px-3 py-1 text-sm whitespace-nowrap cursor-pointer"
                    } else {
                        "border border-border text-muted rounded-full px-3 py-1 text-sm bg-card whitespace-nowrap cursor-pointer"
                    };
                    let key = filter.key.clone();
                    let label = filter.label.clone();
                    rsx! {
                        button {
                            key: "{key}",
                            class: "{class}",
                            onclick: move |_| on_change.call(key.clone()),
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
