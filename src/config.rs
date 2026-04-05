use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_url: String,
    pub api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:8181".into(),
            api_key: String::new(),
        }
    }
}

pub static APP_CONFIG: GlobalSignal<AppConfig> = Signal::global(|| AppConfig::default());

pub async fn load_config() {
    let mut eval = document::eval(
        r#"
        // Priority: query params > localStorage > window.__VOYAGE_CONFIG__
        const params = new URLSearchParams(window.location.search);
        const queryConfig = {
            server_url: params.get("server_url") || "",
            api_key: params.get("api_key") || "",
        };

        const localRaw = localStorage.getItem("voyage_config") || "";
        const windowConfig = (typeof window !== "undefined" && window.__VOYAGE_CONFIG__)
            ? JSON.stringify(window.__VOYAGE_CONFIG__)
            : "";

        dioxus.send(JSON.stringify({
            queryConfig,
            localRaw,
            windowConfig,
        }));
        "#,
    );

    if let Ok(payload) = eval.recv::<String>().await {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ConfigPayload {
            query_config: QueryConfig,
            local_raw: String,
            window_config: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct QueryConfig {
            server_url: String,
            api_key: String,
        }

        if let Ok(parsed) = serde_json::from_str::<ConfigPayload>(&payload) {
            if !parsed.query_config.server_url.trim().is_empty() {
                let mut cfg = APP_CONFIG.read().clone();
                cfg.server_url = parsed.query_config.server_url;
                if !parsed.query_config.api_key.is_empty() {
                    cfg.api_key = parsed.query_config.api_key;
                }
                *APP_CONFIG.write() = cfg;
                return;
            }

            if !parsed.local_raw.is_empty() {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&parsed.local_raw) {
                    *APP_CONFIG.write() = config;
                    return;
                }
            }

            if !parsed.window_config.is_empty() {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&parsed.window_config) {
                    *APP_CONFIG.write() = config;
                }
            }
        }
    }
}

pub async fn save_config(url: &str, key: &str) {
    let config = AppConfig {
        server_url: url.to_string(),
        api_key: key.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&config) {
        let js = format!(
            r#"localStorage.setItem("voyage_config", {});"#,
            serde_json::to_string(&json).unwrap_or_default()
        );
        document::eval(&js);
    }
    *APP_CONFIG.write() = config;
}

pub async fn validate_server(url: &str, key: &str) -> Result<(), String> {
    let js = format!(
        r#"
        try {{
            let resp = await fetch("{}/api/v1/trips", {{
                headers: {{ "X-API-Key": "{}" }}
            }});
            if (resp.ok) {{
                dioxus.send("ok");
            }} else {{
                dioxus.send("error:" + resp.status);
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
        Ok(result) if result == "ok" => Ok(()),
        Ok(result) => Err(result.strip_prefix("error:").unwrap_or(&result).to_string()),
        Err(_) => Err("Failed to connect to server".to_string()),
    }
}
