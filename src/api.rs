//! Thin wrapper around the progenitor-generated Voyage API client.
//!
//! Provides a `get_client()` helper that builds a `voyage_api::Client`
//! from the user's saved config (server URL + API key).

use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::config::APP_CONFIG;
use crate::voyage_api;

/// Build a configured API client from the current APP_CONFIG.
pub fn get_client() -> Result<voyage_api::Client, String> {
    let cfg = APP_CONFIG.peek();
    let mut headers = HeaderMap::new();
    if !cfg.api_key.is_empty() {
        let val = HeaderValue::from_str(&cfg.api_key)
            .map_err(|e| format!("Invalid API key header: {e}"))?;
        headers.insert("X-API-Key", val);
    }
    let http = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let base = format!("{}/api/v1", cfg.server_url.trim_end_matches('/'));
    Ok(voyage_api::Client::new_with_client(&base, http))
}
