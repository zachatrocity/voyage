use dioxus::prelude::*;

const FILTERS: &[&str] = &["All", "Flights ✈️", "Hotels 🏨", "Car Rental 🚗", "Cruises 🚢", "Other"];

#[component]
pub fn FilterChips(active: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        div { class: "flex gap-2 px-4 overflow-x-auto pb-2 scrollbar-hide",
            for filter in FILTERS.iter() {
                {
                    let is_active = active == *filter;
                    let class = if is_active {
                        "bg-primary text-white rounded-full px-3 py-1 text-sm whitespace-nowrap cursor-pointer"
                    } else {
                        "border border-border text-muted rounded-full px-3 py-1 text-sm bg-card whitespace-nowrap cursor-pointer"
                    };
                    let label = filter.to_string();
                    let label2 = label.clone();
                    rsx! {
                        button {
                            class: "{class}",
                            onclick: move |_| on_change.call(label.clone()),
                            "{label2}"
                        }
                    }
                }
            }
        }
    }
}
