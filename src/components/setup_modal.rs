use dioxus::prelude::*;

use crate::components::button::Button;
use crate::components::input::Input;
use crate::config::{save_config, validate_server, APP_CONFIG};

#[component]
pub fn SetupModal() -> Element {
    let mut server_url = use_signal(|| String::new());
    let mut api_key = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

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
            match validate_server(&url, &key).await {
                Ok(()) => {
                    save_config(&url, &key).await;
                }
                Err(e) => {
                    error.set(Some(format!("Connection failed: {e}")));
                }
            }
            loading.set(false);
        });
    };

    // Don't render if config is already set
    if !APP_CONFIG().server_url.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/50",
            div { class: "bg-card rounded-2xl shadow-xl w-[90%] max-w-md p-6 mx-4",
                h2 { class: "text-xl font-bold text-foreground mb-1", "Welcome to Voyage" }
                p { class: "text-muted text-sm mb-6", "Connect to your Voyage server to get started." }

                div { class: "flex flex-col gap-4 mb-4",
                    div { class: "flex flex-col gap-1",
                        label { class: "text-sm font-medium text-foreground", "Server URL" }
                        Input {
                            r#type: "url",
                            placeholder: "https://your-server.example.com",
                            value: "{server_url}",
                            oninput: move |e: FormEvent| server_url.set(e.value()),
                        }
                    }
                    div { class: "flex flex-col gap-1",
                        label { class: "text-sm font-medium text-foreground", "API Key" }
                        Input {
                            r#type: "password",
                            placeholder: "Enter your API key",
                            value: "{api_key}",
                            oninput: move |e: FormEvent| api_key.set(e.value()),
                        }
                    }
                }

                if let Some(err) = error() {
                    p { class: "text-cta text-sm mb-4", "{err}" }
                }

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
}
