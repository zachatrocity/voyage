use dioxus::prelude::*;

use crate::notification::{NotificationKind, NOTIFICATION};

#[component]
pub fn Toast() -> Element {
    let notification = NOTIFICATION();

    // Auto-dismiss after 4 seconds whenever a notification appears
    use_effect(move || {
        if NOTIFICATION().is_some() {
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(4_000).await;
                *NOTIFICATION.write() = None;
            });
        }
    });

    match notification {
        Some(notif) => {
            let bg_class = match notif.kind {
                NotificationKind::Error => "bg-cta",
                NotificationKind::Success => "bg-green-600",
                NotificationKind::Info => "bg-primary",
            };

            rsx! {
                div { class: "fixed bottom-20 left-4 right-4 z-50 {bg_class} text-white rounded-xl shadow-lg px-4 py-3 flex flex-row items-center gap-3",
                    span { class: "flex-1 text-sm", "{notif.message}" }
                    button {
                        class: "text-white opacity-80 hover:opacity-100 font-bold text-lg leading-none",
                        onclick: move |_| { *NOTIFICATION.write() = None; },
                        "\u{00D7}"
                    }
                }
            }
        }
        None => rsx! {},
    }
}
