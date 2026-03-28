use dioxus::prelude::*;

#[component]
pub fn DiscoveryBanner(count: usize) -> Element {
    if count == 0 {
        return rsx! {};
    }

    rsx! {
        div { class: "mx-4 mb-2 rounded-xl bg-primary/10 border border-primary/30 px-4 py-3 flex items-center justify-between",
            span { class: "text-sm text-primary font-medium",
                "We found {count} travel emails — review?"
            }
            button { class: "text-xs bg-cta text-white px-3 py-1.5 rounded-full font-medium",
                "Review"
            }
        }
    }
}
