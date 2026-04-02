use dioxus::prelude::*;

use crate::components::app_header::AppHeader;
use crate::components::bottom_nav::BottomNavBar;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::config::{save_config, validate_server, APP_CONFIG};

#[component]
pub fn Settings() -> Element {
    let config = APP_CONFIG();
    let mut server_url = use_signal(|| config.server_url.clone());
    let mut api_key = use_signal(|| config.api_key.clone());
    let mut error = use_signal(|| Option::<String>::None);
    let mut success = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let masked_key = {
        let key = config.api_key.clone();
        if key.len() > 4 {
            format!("{}****", &key[..4])
        } else if !key.is_empty() {
            "****".to_string()
        } else {
            String::new()
        }
    };

    let on_save = move |_| {
        spawn(async move {
            let url = server_url();
            let key = api_key();
            if url.is_empty() {
                error.set(Some("Server URL is required".to_string()));
                return;
            }
            loading.set(true);
            error.set(None);
            success.set(false);
            match validate_server(&url, &key).await {
                Ok(()) => {
                    save_config(&url, &key).await;
                    success.set(true);
                }
                Err(e) => {
                    error.set(Some(format!("Connection failed: {e}")));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "flex flex-col min-h-screen bg-background",
            AppHeader {}
            div { class: "flex-1 overflow-y-auto px-4 pt-4 pb-20",
                h1 { class: "text-2xl font-bold text-foreground mb-6", "Settings" }

                div { class: "bg-card rounded-xl border border-border p-4 mb-4",
                    h2 { class: "text-lg font-semibold text-foreground mb-4", "Server Connection" }

                    div { class: "flex flex-col gap-4",
                        div { class: "flex flex-col gap-1",
                            label { class: "text-sm font-medium text-foreground", "Server URL" }
                            Input {
                                r#type: "url",
                                placeholder: "https://your-server.example.com",
                                value: "{server_url}",
                                oninput: move |e: FormEvent| {
                                    server_url.set(e.value());
                                    success.set(false);
                                },
                            }
                        }
                        div { class: "flex flex-col gap-1",
                            label { class: "text-sm font-medium text-foreground", "API Key" }
                            if !masked_key.is_empty() {
                                p { class: "text-muted text-xs", "Current: {masked_key}" }
                            }
                            Input {
                                r#type: "password",
                                placeholder: "Enter new API key",
                                value: "{api_key}",
                                oninput: move |e: FormEvent| {
                                    api_key.set(e.value());
                                    success.set(false);
                                },
                            }
                        }
                    }

                    if let Some(err) = error() {
                        p { class: "text-cta text-sm mt-3", "{err}" }
                    }
                    if success() {
                        p { class: "text-primary text-sm mt-3", "Connected successfully!" }
                    }

                    div { class: "mt-4",
                        Button {
                            onclick: on_save,
                            class: "w-full bg-cta text-white rounded-lg py-3 font-semibold",
                            if loading() {
                                "Connecting..."
                            } else {
                                "Save & Connect"
                            }
                        }
                    }
                }
            }
            BottomNavBar { active_tab: "settings".to_string() }
        }
    }
}
