use std::collections::HashMap;

use dioxus::prelude::*;

use crate::api::{
    get_classifiers, reset_classifiers, update_classifiers, CategoryRule, ClassifiersConfig,
};
use crate::components::app_header::AppHeader;
use crate::components::bottom_nav::BottomNavBar;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::config::{save_config, validate_server, APP_CONFIG};
use crate::notification::{notify_error, notify_success};

fn parse_csv(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn to_csv(items: &[String]) -> String {
    items.join(", ")
}

#[component]
pub fn Settings() -> Element {
    let config = APP_CONFIG();
    let mut server_url = use_signal(|| config.server_url.clone());
    let mut api_key = use_signal(|| config.api_key.clone());
    let mut loading = use_signal(|| false);

    let mut classifier_loading = use_signal(|| true);
    let mut classifier_saving = use_signal(|| false);
    let mut classifiers = use_signal(|| ClassifiersConfig {
        categories: HashMap::new(),
    });
    let mut new_category = use_signal(String::new);

    use_effect(move || {
        spawn(async move {
            match get_classifiers().await {
                Ok(cfg) => classifiers.set(cfg),
                Err(e) => notify_error(format!("Failed to load classifiers: {e}")),
            }
            classifier_loading.set(false);
        });
    });

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

    let on_save_connection = move |_| {
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
                    notify_success("Connected successfully!");
                }
                Err(e) => notify_error(format!("Connection failed: {e}")),
            }
            loading.set(false);
        });
    };

    let on_save_classifiers = move |_| {
        spawn(async move {
            classifier_saving.set(true);
            let cfg = classifiers();
            match update_classifiers(&cfg).await {
                Ok(saved) => {
                    classifiers.set(saved);
                    notify_success("Classifier settings saved");
                }
                Err(e) => notify_error(format!("Failed to save classifiers: {e}")),
            }
            classifier_saving.set(false);
        });
    };

    let on_reset_classifiers = move |_| {
        spawn(async move {
            classifier_saving.set(true);
            match reset_classifiers().await {
                Ok(defaults) => {
                    classifiers.set(defaults);
                    notify_success("Classifier settings reset to defaults");
                }
                Err(e) => notify_error(format!("Failed to reset classifiers: {e}")),
            }
            classifier_saving.set(false);
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
                                oninput: move |e: FormEvent| server_url.set(e.value()),
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
                                oninput: move |e: FormEvent| api_key.set(e.value()),
                            }
                        }
                    }

                    div { class: "mt-4",
                        Button {
                            onclick: on_save_connection,
                            class: "w-full bg-cta text-white rounded-lg py-3 font-semibold",
                            if loading() { "Connecting..." } else { "Save & Connect" }
                        }
                    }
                }

                div { class: "bg-card rounded-xl border border-border p-4 mb-4",
                    h2 { class: "text-lg font-semibold text-foreground mb-1", "Classifier Rules" }
                    p { class: "text-sm text-muted mb-4", "Edit domains and subject keywords per category. Use comma-separated values." }

                    if classifier_loading() {
                        p { class: "text-sm text-muted", "Loading classifier settings..." }
                    } else {
                        {
                            let mut category_names = classifiers().categories.keys().cloned().collect::<Vec<_>>();
                            category_names.sort();
                            rsx! {
                                div { class: "flex flex-col gap-3",
                                    for category in category_names {
                                        {
                                            let rule = classifiers().categories.get(&category).cloned().unwrap_or(CategoryRule { domains: vec![], subject_keywords: vec![] });
                                            let domains_csv = to_csv(&rule.domains);
                                            let keywords_csv = to_csv(&rule.subject_keywords);
                                            rsx! {
                                                div { key: "{category}", class: "border border-border rounded-lg p-3",
                                                    div { class: "flex items-center justify-between mb-2",
                                                        h3 { class: "font-semibold text-foreground", "{category}" }
                                                        button {
                                                            class: "text-xs text-red-400",
                                                            onclick: {
                                                                let category = category.clone();
                                                                move |_| {
                                                                    let mut cfg = classifiers();
                                                                    cfg.categories.remove(&category);
                                                                    classifiers.set(cfg);
                                                                }
                                                            },
                                                            "Remove"
                                                        }
                                                    }

                                                    div { class: "mb-2",
                                                        label { class: "text-xs text-muted", "Domains" }
                                                        Input {
                                                            r#type: "text",
                                                            placeholder: "delta.com, united.com",
                                                            value: "{domains_csv}",
                                                            oninput: {
                                                                let category = category.clone();
                                                                move |e: FormEvent| {
                                                                    let mut cfg = classifiers();
                                                                    let entry = cfg.categories.entry(category.clone()).or_insert(CategoryRule { domains: vec![], subject_keywords: vec![] });
                                                                    entry.domains = parse_csv(&e.value());
                                                                    classifiers.set(cfg);
                                                                }
                                                            }
                                                        }
                                                    }

                                                    div {
                                                        label { class: "text-xs text-muted", "Subject keywords" }
                                                        Input {
                                                            r#type: "text",
                                                            placeholder: "boarding pass, itinerary",
                                                            value: "{keywords_csv}",
                                                            oninput: {
                                                                let category = category.clone();
                                                                move |e: FormEvent| {
                                                                    let mut cfg = classifiers();
                                                                    let entry = cfg.categories.entry(category.clone()).or_insert(CategoryRule { domains: vec![], subject_keywords: vec![] });
                                                                    entry.subject_keywords = parse_csv(&e.value());
                                                                    classifiers.set(cfg);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "mt-4 border-t border-border pt-4",
                            label { class: "text-xs text-muted", "Add category" }
                            div { class: "flex gap-2 mt-1",
                                Input {
                                    r#type: "text",
                                    placeholder: "example: train",
                                    value: "{new_category}",
                                    oninput: move |e: FormEvent| new_category.set(e.value()),
                                }
                                Button {
                                    class: "px-4",
                                    onclick: move |_| {
                                        let name = new_category().trim().to_lowercase().replace(' ', "_");
                                        if name.is_empty() {
                                            return;
                                        }
                                        let mut cfg = classifiers();
                                        cfg.categories.entry(name).or_insert(CategoryRule { domains: vec![], subject_keywords: vec![] });
                                        classifiers.set(cfg);
                                        new_category.set(String::new());
                                    },
                                    "Add"
                                }
                            }
                        }

                        div { class: "mt-4 flex gap-2",
                            Button {
                                onclick: on_save_classifiers,
                                class: "flex-1 bg-cta text-white rounded-lg py-3 font-semibold",
                                if classifier_saving() { "Saving..." } else { "Save Classifiers" }
                            }
                            Button {
                                onclick: on_reset_classifiers,
                                class: "flex-1 rounded-lg py-3 font-semibold",
                                "Reset to Defaults"
                            }
                        }
                    }
                }
            }
            BottomNavBar { active_tab: "settings".to_string() }
        }
    }
}
