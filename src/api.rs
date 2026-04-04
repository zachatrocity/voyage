use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::config::APP_CONFIG;
use crate::types::Category;

/// Convenience re-export for views that need the count
impl SearchResults {
    pub fn emails(&self) -> &[EmailResult] {
        &self.results
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ApiError {
    Network(String),
    Decode(String),
    Server { status: u16, message: String },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {msg}"),
            ApiError::Decode(msg) => write!(f, "Decode error: {msg}"),
            ApiError::Server { status, message } => {
                write!(f, "Server error ({status}): {message}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailResult {
    pub message_id: String,
    #[serde(default)]
    pub thread_id: String,
    pub subject: String,
    pub from: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub body_preview: String,
    #[serde(default)]
    pub category: Category,
    #[serde(default)]
    pub trip_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub query: String,
    pub count: usize,
    pub results: Vec<EmailResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripResponse {
    pub id: String,
    pub name: String,
    pub date_range: String,
    pub email_count: usize,
    pub confirmed_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripsResponse {
    pub trips: Vec<TripResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripEmailItem {
    pub id: String,
    pub subject: String,
    pub sender: String,
    pub date: String,
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripEmailsResponse {
    pub trip_id: String,
    pub emails: Vec<TripEmailItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociateTripResponse {
    pub email_id: String,
    pub trip_id: String,
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct CreateTripBody {
    name: String,
    date_range: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn client() -> Result<(reqwest::Client, String), ApiError> {
    let cfg = APP_CONFIG.read();
    let mut headers = HeaderMap::new();
    if !cfg.api_key.is_empty() {
        let val = HeaderValue::from_str(&cfg.api_key)
            .map_err(|e| ApiError::Network(e.to_string()))?;
        headers.insert("X-API-Key", val);
    }
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| ApiError::Network(e.to_string()))?;
    let base = format!("{}/api/v1", cfg.server_url.trim_end_matches('/'));
    Ok((client, base))
}

async fn handle_response<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<T, ApiError> {
    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let message = resp.text().await.unwrap_or_default();
        return Err(ApiError::Server { status, message });
    }
    resp.json::<T>()
        .await
        .map_err(|e| ApiError::Decode(e.to_string()))
}

// ---------------------------------------------------------------------------
// Public API functions
// ---------------------------------------------------------------------------

pub async fn search_emails(query: &str, limit: Option<u32>) -> Result<SearchResults, ApiError> {
    let (client, base) = client()?;
    // Backend requires non-empty q param; use "*" to list all emails
    let q = if query.is_empty() { "*" } else { query };
    let mut url = format!("{base}/search?q={}", urlencoding::encode(q));
    if let Some(l) = limit {
        url.push_str(&format!("&limit={l}"));
    }
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn get_email(id: &str) -> Result<EmailResult, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/email/{id}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn tag_email(id: &str, tag: &str) -> Result<EmailResult, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/email/{id}/tags/{tag}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn associate_email_trip(
    email_id: &str,
    trip_id: &str,
) -> Result<AssociateTripResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/email/{email_id}/trip/{trip_id}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn list_trips() -> Result<TripsResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/trips"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn create_trip(name: &str, date_range: &str) -> Result<TripResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/trips"))
        .json(&CreateTripBody {
            name: name.to_string(),
            date_range: date_range.to_string(),
        })
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn get_trip_emails(trip_id: &str) -> Result<TripEmailsResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/trips/{trip_id}/emails"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}
