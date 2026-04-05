use dioxus::prelude::*;

use crate::notification::{NotificationKind, NOTIFICATION};

#[component]
pub fn Toast() -> Element {
    // Track whether a notification is active so we can spawn a single dismiss timer
    let mut timer_running = use_signal(|| false);

    let notification = NOTIFICATION();

    // Spawn auto-dismiss timer only when a new notification arrives and no timer is active
    if notification.is_some() && !timer_running() {
        timer_running.set(true);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(4_000).await;
            *NOTIFICATION.write() = None;
            timer_running.set(false);
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
                            timer_running.set(false);
                        },
                        "\u{00D7}"
                    }
                }
            }
        }
        None => rsx! {},
    }
}
