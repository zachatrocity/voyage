use dioxus::prelude::*;

#[component]
pub fn BottomSheet(children: Element) -> Element {
    rsx! {
        div { class: "fixed inset-x-0 bottom-0 bg-card rounded-t-2xl shadow-2xl border-t border-border",
            // Drag handle
            div { class: "flex justify-center pt-3 pb-1",
                div { class: "w-10 h-1 rounded-full bg-border" }
            }
            // Content slot
            div { class: "px-4 pb-8",
                {children}
            }
        }
    }
}
