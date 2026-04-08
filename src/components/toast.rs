use dioxus::prelude::*;

use crate::notification::{NotificationKind, NOTIFICATION};

#[component]
pub fn Toast() -> Element {
    let notification = NOTIFICATION();

    // Important: never write signals during render. Schedule dismiss in an effect.
    {
        let notification_for_effect = notification.clone();
        use_effect(move || {
            if notification_for_effect.is_some() {
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(4_000).await;
                    *NOTIFICATION.write() = None;
                });
            }
        });
    }

    match notification {
        Some(notif) => {
            let toast_class = match notif.kind {
                NotificationKind::Error => {
                    "fixed bottom-20 left-4 right-4 z-50 bg-cta text-white rounded-xl shadow-lg px-4 py-3 flex flex-row items-center gap-3"
                }
                NotificationKind::Success => {
                    "fixed bottom-20 left-4 right-4 z-50 bg-primary text-white rounded-xl shadow-lg px-4 py-3 flex flex-row items-center gap-3"
                }
                NotificationKind::Info => {
                    "fixed bottom-20 left-4 right-4 z-50 bg-card text-foreground border border-border rounded-xl shadow-lg px-4 py-3 flex flex-row items-center gap-3"
                }
            };

            let close_btn_class = match notif.kind {
                NotificationKind::Info => {
                    "text-foreground opacity-70 hover:opacity-100 font-bold text-lg leading-none"
                }
                _ => "text-white opacity-80 hover:opacity-100 font-bold text-lg leading-none",
            };

            rsx! {
                div { class: "{toast_class}",
                    span { class: "flex-1 text-sm", "{notif.message}" }
                    button {
                        class: "{close_btn_class}",
                        onclick: move |_| {
                            *NOTIFICATION.write() = None;
                        },
                        "\u{00D7}"
                    }
                }
            }
        }
        None => rsx! {},
    }
}
