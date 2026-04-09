use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdChevronDown, LdChevronRight, LdPlus, LdTrash2, LdX};
use dioxus_free_icons::Icon;

use crate::components::app_header::AppHeader;
use crate::components::bottom_nav::BottomNavBar;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::config::{save_config, validate_server, APP_CONFIG};
use crate::generated::api_types::{CategoryRule, ClassifiersConfig};
use crate::notification::{notify_error, notify_success};

// ── helpers ──────────────────────────────────────────────────────────────────

/// "car_rental" → "Car Rental"
fn title_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ── API helpers ───────────────────────────────────────────────────────────────

async fn fetch_classifiers() -> Result<ClassifiersConfig, String> {
    let url = APP_CONFIG().server_url;
    let key = APP_CONFIG().api_key;
    let js = format!(
        r#"
        try {{
            let resp = await fetch("{}/api/v1/classifiers", {{
                headers: {{ "X-API-Key": "{}" }}
            }});
            let text = await resp.text();
            if (resp.ok) {{
                dioxus.send(text);
            }} else {{
                dioxus.send("error:" + resp.status + ":" + text);
            }}
        }} catch (e) {{
            dioxus.send("error:" + e.message);
        }}
        "#,
        url.trim_end_matches('/'),
        key
    );
    let mut eval = document::eval(&js);
    match eval.recv::<String>().await {
        Ok(result) if !result.starts_with("error:") => {
            serde_json::from_str::<ClassifiersConfig>(&result)
                .map_err(|e| format!("Parse error: {e}"))
        }
        Ok(result) => Err(result
            .strip_prefix("error:")
            .unwrap_or(&result)
            .to_string()),
        Err(_) => Err("Failed to communicate with server".to_string()),
    }
}

async fn save_classifiers(config: &ClassifiersConfig) -> Result<(), String> {
    let url = APP_CONFIG().server_url;
    let key = APP_CONFIG().api_key;
    let body = serde_json::to_string(config).map_err(|e| e.to_string())?;
    let body_escaped = serde_json::to_string(&body).unwrap_or_default();
    let js = format!(
        r#"
        try {{
            let resp = await fetch("{}/api/v1/classifiers", {{
                method: "PUT",
                headers: {{
                    "X-API-Key": "{}",
                    "Content-Type": "application/json"
                }},
                body: {}
            }});
            if (resp.ok) {{
                dioxus.send("ok");
            }} else {{
                let text = await resp.text();
                dioxus.send("error:" + resp.status + ":" + text);
            }}
        }} catch (e) {{
            dioxus.send("error:" + e.message);
        }}
        "#,
        url.trim_end_matches('/'),
        key,
        body_escaped
    );
    let mut eval = document::eval(&js);
    match eval.recv::<String>().await {
        Ok(result) if result == "ok" => Ok(()),
        Ok(result) => Err(result
            .strip_prefix("error:")
            .unwrap_or(&result)
            .to_string()),
        Err(_) => Err("Failed to communicate with server".to_string()),
    }
}

async fn reset_classifiers() -> Result<ClassifiersConfig, String> {
    let url = APP_CONFIG().server_url;
    let key = APP_CONFIG().api_key;
    let js = format!(
        r#"
        try {{
            let resp = await fetch("{}/api/v1/classifiers/reset", {{
                method: "POST",
                headers: {{ "X-API-Key": "{}" }}
            }});
            let text = await resp.text();
            if (resp.ok) {{
                dioxus.send(text);
            }} else {{
                dioxus.send("error:" + resp.status + ":" + text);
            }}
        }} catch (e) {{
            dioxus.send("error:" + e.message);
        }}
        "#,
        url.trim_end_matches('/'),
        key
    );
    let mut eval = document::eval(&js);
    match eval.recv::<String>().await {
        Ok(result) if !result.starts_with("error:") => {
            serde_json::from_str::<ClassifiersConfig>(&result)
                .map_err(|e| format!("Parse error: {e}"))
        }
        Ok(result) => Err(result
            .strip_prefix("error:")
            .unwrap_or(&result)
            .to_string()),
        Err(_) => Err("Failed to communicate with server".to_string()),
    }
}

// ── Chip component ────────────────────────────────────────────────────────────

#[component]
fn Chip(label: String, on_remove: EventHandler<()>) -> Element {
    rsx! {
        span { class: "inline-flex items-center gap-1 bg-muted/20 text-foreground text-sm rounded-full px-3 py-1 border border-border",
            span { "{label}" }
            button {
                class: "text-muted hover:text-foreground transition-colors ml-1",
                onclick: move |_| on_remove(()),
                Icon { icon: LdX, width: 12, height: 12 }
            }
        }
    }
}

// ── ChipList component ────────────────────────────────────────────────────────

#[component]
fn ChipList(
    label: String,
    items: Vec<String>,
    on_remove: EventHandler<usize>,
    on_add: EventHandler<String>,
) -> Element {
    let mut new_value = use_signal(|| String::new());

    let on_add_click = {
        let on_add = on_add.clone();
        move |_| {
            let val = new_value().trim().to_string();
            if !val.is_empty() {
                on_add(val);
                new_value.set(String::new());
            }
        }
    };

    let on_keydown = {
        let on_add = on_add.clone();
        move |e: KeyboardEvent| {
            if e.key() == Key::Enter {
                let val = new_value().trim().to_string();
                if !val.is_empty() {
                    on_add(val);
                    new_value.set(String::new());
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-2",
            p { class: "text-sm font-medium text-foreground", "{label}" }
            if items.is_empty() {
                p { class: "text-muted text-xs italic", "None added yet." }
            } else {
                div { class: "flex flex-wrap gap-2",
                    for (idx, item) in items.iter().enumerate() {
                        Chip {
                            key: "{idx}",
                            label: item.clone(),
                            on_remove: move |_| on_remove(idx),
                        }
                    }
                }
            }
            div { class: "flex gap-2",
                Input {
                    r#type: "text",
                    placeholder: "Add {label}...",
                    value: "{new_value}",
                    oninput: move |e: FormEvent| new_value.set(e.value()),
                    onkeydown: on_keydown,
                }
                Button {
                    onclick: on_add_click,
                    class: "shrink-0",
                    Icon { icon: LdPlus, width: 16, height: 16 }
                }
            }
        }
    }
}

// ── CategorySection component ─────────────────────────────────────────────────

#[component]
fn CategorySection(
    name: String,
    rule: CategoryRule,
    on_change: EventHandler<CategoryRule>,
    on_delete: EventHandler<()>,
) -> Element {
    let mut expanded = use_signal(|| false);

    let domains = rule.domains.clone().unwrap_or_default();
    let keywords = rule.subject_keywords.clone().unwrap_or_default();

    let on_remove_domain = {
        let rule = rule.clone();
        let on_change = on_change.clone();
        move |idx: usize| {
            let mut new_rule = rule.clone();
            let mut doms = new_rule.domains.clone().unwrap_or_default();
            if idx < doms.len() {
                doms.remove(idx);
            }
            new_rule.domains = Some(doms);
            on_change(new_rule);
        }
    };

    let on_add_domain = {
        let rule = rule.clone();
        let on_change = on_change.clone();
        move |val: String| {
            let mut new_rule = rule.clone();
            let mut doms = new_rule.domains.clone().unwrap_or_default();
            if !doms.contains(&val) {
                doms.push(val);
            }
            new_rule.domains = Some(doms);
            on_change(new_rule);
        }
    };

    let on_remove_keyword = {
        let rule = rule.clone();
        let on_change = on_change.clone();
        move |idx: usize| {
            let mut new_rule = rule.clone();
            let mut kws = new_rule.subject_keywords.clone().unwrap_or_default();
            if idx < kws.len() {
                kws.remove(idx);
            }
            new_rule.subject_keywords = Some(kws);
            on_change(new_rule);
        }
    };

    let on_add_keyword = {
        let rule = rule.clone();
        let on_change = on_change.clone();
        move |val: String| {
            let mut new_rule = rule.clone();
            let mut kws = new_rule.subject_keywords.clone().unwrap_or_default();
            if !kws.contains(&val) {
                kws.push(val);
            }
            new_rule.subject_keywords = Some(kws);
            on_change(new_rule);
        }
    };

    let display_name = title_case(&name);
    let counts_label = format!("({} domains, {} keywords)", domains.len(), keywords.len());

    rsx! {
        div { class: "border border-border rounded-lg overflow-hidden mb-2",
            // Header row
            div {
                class: "flex items-center justify-between px-3 py-2 bg-muted/10 cursor-pointer",
                onclick: move |_| expanded.set(!expanded()),
                div { class: "flex items-center gap-2",
                    if expanded() {
                        Icon { icon: LdChevronDown, width: 16, height: 16, class: "text-muted" }
                    } else {
                        Icon { icon: LdChevronRight, width: 16, height: 16, class: "text-muted" }
                    }
                    span { class: "font-medium text-foreground", "{display_name}" }
                    span { class: "text-xs text-muted", "{counts_label}" }
                }
                button {
                    class: "text-muted hover:text-destructive transition-colors p-1",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_delete(());
                    },
                    Icon { icon: LdTrash2, width: 16, height: 16 }
                }
            }
            // Expandable body
            if expanded() {
                div { class: "px-3 py-3 flex flex-col gap-4",
                    ChipList {
                        label: "Domains".to_string(),
                        items: domains,
                        on_remove: on_remove_domain,
                        on_add: on_add_domain,
                    }
                    ChipList {
                        label: "Subject Keywords".to_string(),
                        items: keywords,
                        on_remove: on_remove_keyword,
                        on_add: on_add_keyword,
                    }
                }
            }
        }
    }
}

// ── AddCategoryModal component ─────────────────────────────────────────────────

#[component]
fn AddCategoryModal(on_confirm: EventHandler<String>, on_cancel: EventHandler<()>) -> Element {
    let mut name = use_signal(|| String::new());

    let on_confirm_click = {
        let on_confirm = on_confirm.clone();
        move |_| {
            let val = name()
                .trim()
                .to_lowercase()
                .replace(' ', "_");
            if !val.is_empty() {
                on_confirm(val);
            }
        }
    };

    rsx! {
        div { class: "fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4",
            div { class: "bg-card rounded-xl border border-border p-5 w-full max-w-sm flex flex-col gap-4",
                h3 { class: "text-lg font-semibold text-foreground", "Add Category" }
                p { class: "text-muted text-sm", "Enter a category name (lowercase, use underscores for spaces)." }
                Input {
                    r#type: "text",
                    placeholder: "e.g. rail or theme_park",
                    value: "{name}",
                    oninput: move |e: FormEvent| name.set(e.value()),
                    onkeydown: {
                        let on_confirm = on_confirm.clone();
                        move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                let val = name()
                                    .trim()
                                    .to_lowercase()
                                    .replace(' ', "_");
                                if !val.is_empty() {
                                    on_confirm(val);
                                }
                            }
                        }
                    },
                }
                div { class: "flex gap-2",
                    Button {
                        onclick: move |_| on_cancel(()),
                        class: "flex-1",
                        "Cancel"
                    }
                    Button {
                        onclick: on_confirm_click,
                        class: "flex-1 bg-cta text-white",
                        "Add"
                    }
                }
            }
        }
    }
}

// ── ClassifiersCard component ─────────────────────────────────────────────────

#[component]
fn ClassifiersCard() -> Element {
    // HashMap<category_name, CategoryRule>
    let mut categories: Signal<HashMap<String, CategoryRule>> =
        use_signal(|| HashMap::new());
    let mut loading = use_signal(|| true);
    let mut saving = use_signal(|| false);
    let mut show_add_modal = use_signal(|| false);

    // Fetch on mount
    use_effect(move || {
        spawn(async move {
            match fetch_classifiers().await {
                Ok(config) => {
                    if let Some(cats) = config.categories {
                        categories.set(cats);
                    }
                }
                Err(e) => {
                    notify_error(format!("Failed to load classifiers: {e}"));
                }
            }
            loading.set(false);
        });
    });

    let on_save = move |_| {
        let cats = categories();
        spawn(async move {
            saving.set(true);
            let config = ClassifiersConfig {
                categories: Some(cats),
            };
            match save_classifiers(&config).await {
                Ok(()) => notify_success("Classifier settings saved!"),
                Err(e) => notify_error(format!("Save failed: {e}")),
            }
            saving.set(false);
        });
    };

    let on_reset = move |_| {
        spawn(async move {
            saving.set(true);
            match reset_classifiers().await {
                Ok(config) => {
                    if let Some(cats) = config.categories {
                        categories.set(cats);
                    }
                    notify_success("Reset to defaults!");
                }
                Err(e) => {
                    notify_error(format!("Reset failed: {e}"));
                }
            }
            saving.set(false);
        });
    };

    // Sorted category names for stable rendering order
    let sorted_names = {
        let mut names: Vec<String> = categories().keys().cloned().collect();
        names.sort();
        names
    };

    rsx! {
        div { class: "bg-card rounded-xl border border-border p-4 mb-4",
            h2 { class: "text-lg font-semibold text-foreground mb-1", "Email Classifiers" }
            p { class: "text-muted text-sm mb-4", "Control which domains and subject keywords route emails into each travel category." }

            if loading() {
                div { class: "flex items-center justify-center py-8",
                    span { class: "text-muted text-sm", "Loading classifiers…" }
                }
            } else {
                div { class: "flex flex-col",
                    for name in sorted_names.iter() {
                        {
                            let name_key = name.clone();
                            let name_for_change = name.clone();
                            let name_for_delete = name.clone();
                            let rule = categories().get(name).cloned().unwrap_or(CategoryRule {
                                domains: Some(vec![]),
                                subject_keywords: Some(vec![]),
                            });
                            rsx! {
                                CategorySection {
                                    key: "{name_key}",
                                    name: name_key.clone(),
                                    rule: rule,
                                    on_change: move |updated: CategoryRule| {
                                        let n = name_for_change.clone();
                                        categories.with_mut(|cats| {
                                            cats.insert(n.clone(), updated);
                                        });
                                    },
                                    on_delete: move |_| {
                                        let n = name_for_delete.clone();
                                        categories.with_mut(|cats| {
                                            cats.remove(&n);
                                        });
                                    },
                                }
                            }
                        }
                    }

                    // Add category button
                    button {
                        class: "flex items-center gap-2 text-primary text-sm mt-1 mb-4 hover:underline self-start",
                        onclick: move |_| show_add_modal.set(true),
                        Icon { icon: LdPlus, width: 14, height: 14 }
                        "Add Category"
                    }

                    // Save / Reset actions
                    div { class: "flex gap-2",
                        Button {
                            onclick: on_reset,
                            class: "flex-1",
                            if saving() { "Working…" } else { "Reset to Defaults" }
                        }
                        Button {
                            onclick: on_save,
                            class: "flex-1 bg-cta text-white",
                            if saving() { "Saving…" } else { "Save Changes" }
                        }
                    }
                }
            }
        }

        // Modal overlay (rendered outside the card flow)
        if show_add_modal() {
            AddCategoryModal {
                on_confirm: move |name: String| {
                    if !name.is_empty() {
                        categories.with_mut(|cats| {
                            cats.entry(name).or_insert(CategoryRule {
                                domains: Some(vec![]),
                                subject_keywords: Some(vec![]),
                            });
                        });
                    }
                    show_add_modal.set(false);
                },
                on_cancel: move |_| show_add_modal.set(false),
            }
        }
    }
}

// ── Settings view ─────────────────────────────────────────────────────────────

#[component]
pub fn Settings() -> Element {
    let config = APP_CONFIG();
    let mut server_url = use_signal(|| config.server_url.clone());
    let mut api_key = use_signal(|| config.api_key.clone());
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
                notify_error("Server URL is required");
                return;
            }
            loading.set(true);
            match validate_server(&url, &key).await {
                Ok(()) => {
                    save_config(&url, &key).await;
                    notify_success("Connected successfully!");
                }
                Err(e) => {
                    notify_error(format!("Connection failed: {e}"));
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

                // ── Server Connection card ──────────────────────────────────
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
                                },
                            }
                        }
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

                // ── Email Classifiers card ──────────────────────────────────
                ClassifiersCard {}
            }
            BottomNavBar { active_tab: "settings".to_string() }
        }
    }
}
