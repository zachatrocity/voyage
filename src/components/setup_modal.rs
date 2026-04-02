use dioxus::prelude::*;

use crate::components::button::Button;
use crate::components::input::Input;
use crate::config::{save_config, validate_server, APP_CONFIG};
use crate::notification::notify_error;

#[component]
pub fn SetupModal() -> Element {
    let mut server_url = use_signal(|| String::new());
    let mut api_key = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let on_save = move |_| {
        spawn(async move {
            let url = server_url();
            let key = api_key();
            if url.is_empty() {
                notify_error("Server URL is required");
                return;
            }
            loading.set(true);
            match validate_server(&url, &key).await {
                Ok(()) => {
                    save_config(&url, &key).await;
                }
                Err(e) => {
                    notify_error(format!("Connection failed: {e}"));
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
        div { class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm overflow-hidden",
            div { class: "bg-card rounded-2xl shadow-2xl w-[90%] max-w-md p-6 mx-4 relative",
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
