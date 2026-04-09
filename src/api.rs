use dioxus::prelude::ReadableExt;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::config::APP_CONFIG;
use crate::generated::api_types as gen;
use crate::types::Category;

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
// App-facing response types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailResult {
    pub id: String,
    pub subject: String,
    pub sender: String,
    pub sender_email: String,
    pub date: String,
    pub body_preview: String,
    pub category: Category,
    pub tags: Vec<String>,
    pub trip_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub emails: Vec<EmailResult>,
    pub total: usize,
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
    pub tags: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailContentResponse {
    pub message_id: String,
    pub body: String,
    pub html_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRule {
    pub domains: Vec<String>,
    pub subject_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiersConfig {
    pub categories: std::collections::HashMap<String, CategoryRule>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn diag_trips_config(call: &str) {
    let cfg = APP_CONFIG.read();
    eprintln!(
        "[diag][api::{call}] config server_url={} api_key_present={}",
        cfg.server_url,
        !cfg.api_key.is_empty()
    );
}

fn diag_req_start(call: &str, method: &str, url: &str) {
    eprintln!("[diag][api::{call}] request start method={method} url={url}");
}

fn diag_req_end(call: &str, method: &str, url: &str, status: u16) {
    eprintln!("[diag][api::{call}] request end method={method} url={url} status={status}");
}

fn diag_req_failure(call: &str, method: &str, url: &str, error: &str) {
    eprintln!("[diag][api::{call}] request failure method={method} url={url} error={error}");
}

fn client() -> Result<(reqwest::Client, String), ApiError> {
    let cfg = APP_CONFIG.read();

    if cfg.server_url.trim().is_empty() {
        return Err(ApiError::Server {
            status: 400,
            message: "Server is not configured yet. Open Settings and connect your server."
                .to_string(),
        });
    }

    let mut headers = HeaderMap::new();
    if !cfg.api_key.is_empty() {
        let val =
            HeaderValue::from_str(&cfg.api_key).map_err(|e| ApiError::Network(e.to_string()))?;
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

fn parse_category(value: Option<String>) -> Category {
    match value.unwrap_or_default().to_ascii_lowercase().as_str() {
        "flight" => Category::Flight,
        "hotel" => Category::Hotel,
        "car_rental" | "carrental" | "car-rental" => Category::CarRental,
        "cruise" => Category::Cruise,
        "activity" => Category::Activity,
        _ => Category::Other,
    }
}

fn map_email(raw: gen::EmailResult) -> EmailResult {
    EmailResult {
        id: raw
            .message_id
            .or(raw.thread_id)
            .unwrap_or_else(|| "unknown".to_string()),
        subject: raw.subject.unwrap_or_default(),
        sender: raw.from.unwrap_or_default(),
        sender_email: String::new(),
        date: raw.date.unwrap_or_default(),
        body_preview: raw.body_preview.unwrap_or_default(),
        category: parse_category(raw.category),
        tags: raw.tags.unwrap_or_default(),
        trip_id: raw.trip_id,
    }
}

fn map_trip(raw: gen::TripResponse) -> TripResponse {
    TripResponse {
        id: raw.id.unwrap_or_default(),
        name: raw.name.unwrap_or_default(),
        date_range: raw.date_range.unwrap_or_default(),
        email_count: raw.email_count.unwrap_or(0).max(0) as usize,
        confirmed_count: raw.confirmed_count.unwrap_or(0).max(0) as usize,
    }
}

fn map_classifiers(raw: gen::ClassifierClassifiersConfig) -> ClassifiersConfig {
    let categories = raw
        .categories
        .unwrap_or_default()
        .into_iter()
        .map(|(name, rule)| {
            (
                name,
                CategoryRule {
                    domains: rule.domains.unwrap_or_default(),
                    subject_keywords: rule.subject_keywords.unwrap_or_default(),
                },
            )
        })
        .collect();

    ClassifiersConfig { categories }
}

fn unmap_classifiers(cfg: &ClassifiersConfig) -> gen::ClassifierClassifiersConfig {
    let categories = cfg
        .categories
        .iter()
        .map(|(name, rule)| {
            (
                name.clone(),
                gen::ClassifierCategoryRule {
                    domains: Some(rule.domains.clone()),
                    subject_keywords: Some(rule.subject_keywords.clone()),
                },
            )
        })
        .collect();

    gen::ClassifierClassifiersConfig {
        categories: Some(categories),
    }
}

// ---------------------------------------------------------------------------
// Public API functions
// ---------------------------------------------------------------------------

pub async fn search_emails(query: &str, limit: Option<u32>) -> Result<SearchResults, ApiError> {
    let (client, base) = client()?;
    let mut url = format!("{base}/search?q={}", urlencoding::encode(query));
    if let Some(l) = limit {
        url.push_str(&format!("&limit={l}"));
    }

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::NotmuchSearchResults = handle_response(resp).await?;
    let emails = raw
        .results
        .unwrap_or_default()
        .into_iter()
        .map(map_email)
        .collect::<Vec<_>>();

    Ok(SearchResults {
        total: raw.count.unwrap_or(0).max(0) as usize,
        emails,
    })
}

pub async fn get_email(id: &str) -> Result<EmailResult, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/email/{id}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::EmailResult = handle_response(resp).await?;
    Ok(map_email(raw))
}

pub async fn tag_email(id: &str, tag: &str) -> Result<EmailResult, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/email/{id}/tags/{tag}"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::EmailResult = handle_response(resp).await?;
    Ok(map_email(raw))
}

pub async fn get_email_content(id: &str) -> Result<EmailContentResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/email/{id}/content"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    handle_response(resp).await
}

pub async fn get_classifiers() -> Result<ClassifiersConfig, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/classifiers"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::ClassifierClassifiersConfig = handle_response(resp).await?;
    Ok(map_classifiers(raw))
}

pub async fn update_classifiers(config: &ClassifiersConfig) -> Result<ClassifiersConfig, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .put(format!("{base}/classifiers"))
        .json(&unmap_classifiers(config))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::ClassifierClassifiersConfig = handle_response(resp).await?;
    Ok(map_classifiers(raw))
}

pub async fn reset_classifiers() -> Result<ClassifiersConfig, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/classifiers/reset"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::ClassifierClassifiersConfig = handle_response(resp).await?;
    Ok(map_classifiers(raw))
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

    let raw: gen::AssociateTripResponse = handle_response(resp).await?;
    Ok(AssociateTripResponse {
        email_id: raw.message_id.unwrap_or_else(|| email_id.to_string()),
        trip_id: raw.trip_id.unwrap_or_else(|| trip_id.to_string()),
    })
}

pub async fn list_trips() -> Result<TripsResponse, ApiError> {
    let call = "list_trips";
    let (client, base) = client()?;
    let url = format!("{base}/trips");

    diag_trips_config(call);
    diag_req_start(call, "GET", &url);

    let resp = client.get(&url).send().await.map_err(|e| {
        let msg = e.to_string();
        diag_req_failure(call, "GET", &url, &msg);
        ApiError::Network(msg)
    })?;

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let message = resp.text().await.unwrap_or_default();
        diag_req_failure(
            call,
            "GET",
            &url,
            &format!("status={status} body={message}"),
        );
        return Err(ApiError::Server { status, message });
    }
    diag_req_end(call, "GET", &url, status);

    // Swagger currently models GET /trips as an inline object with
    // additionalProperties: array<tripResponse> (not a named TripsResponse).
    let raw: std::collections::HashMap<String, Vec<gen::TripResponse>> =
        resp.json().await.map_err(|e| {
            let msg = e.to_string();
            diag_req_failure(call, "GET", &url, &format!("decode={msg}"));
            ApiError::Decode(msg)
        })?;
    let trips = raw
        .get("trips")
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(map_trip)
        .collect();

    Ok(TripsResponse { trips })
}

pub async fn create_trip(name: &str, date_range: &str) -> Result<TripResponse, ApiError> {
    let call = "create_trip";
    let (client, base) = client()?;
    let url = format!("{base}/trips");

    diag_trips_config(call);
    diag_req_start(call, "POST", &url);

    let resp = client
        .post(&url)
        .json(&gen::CreateTripBody {
            name: Some(name.to_string()),
            date_range: Some(date_range.to_string()),
        })
        .send()
        .await
        .map_err(|e| {
            let msg = e.to_string();
            diag_req_failure(call, "POST", &url, &msg);
            ApiError::Network(msg)
        })?;

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let message = resp.text().await.unwrap_or_default();
        diag_req_failure(
            call,
            "POST",
            &url,
            &format!("status={status} body={message}"),
        );
        return Err(ApiError::Server { status, message });
    }
    diag_req_end(call, "POST", &url, status);

    // Swagger currently omits a concrete 200 response schema for POST /trips.
    // Backend returns a trip-like object; we decode as TripResponse by contract.
    let raw: gen::TripResponse = resp.json().await.map_err(|e| {
        let msg = e.to_string();
        diag_req_failure(call, "POST", &url, &format!("decode={msg}"));
        ApiError::Decode(msg)
    })?;
    Ok(map_trip(raw))
}

pub async fn delete_trip(trip_id: &str) -> Result<(), ApiError> {
    let call = "delete_trip";
    let (client, base) = client()?;
    let url = format!("{base}/trips/{trip_id}");

    diag_trips_config(call);
    diag_req_start(call, "DELETE", &url);

    let resp = client.delete(&url).send().await.map_err(|e| {
        let msg = e.to_string();
        diag_req_failure(call, "DELETE", &url, &msg);
        ApiError::Network(msg)
    })?;

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let message = resp.text().await.unwrap_or_default();
        diag_req_failure(
            call,
            "DELETE",
            &url,
            &format!("status={status} body={message}"),
        );
        return Err(ApiError::Server { status, message });
    }

    diag_req_end(call, "DELETE", &url, status);
    Ok(())
}

pub async fn get_trip_emails(trip_id: &str) -> Result<TripEmailsResponse, ApiError> {
    let call = "get_trip_emails";
    let (client, base) = client()?;
    let url = format!("{base}/trips/{trip_id}/emails");

    diag_trips_config(call);
    diag_req_start(call, "GET", &url);

    let resp = client.get(&url).send().await.map_err(|e| {
        let msg = e.to_string();
        diag_req_failure(call, "GET", &url, &msg);
        ApiError::Network(msg)
    })?;

    let status = resp.status().as_u16();
    if !resp.status().is_success() {
        let message = resp.text().await.unwrap_or_default();
        diag_req_failure(
            call,
            "GET",
            &url,
            &format!("status={status} body={message}"),
        );
        return Err(ApiError::Server { status, message });
    }
    diag_req_end(call, "GET", &url, status);

    let raw: gen::TripEmailsResponse = resp.json().await.map_err(|e| {
        let msg = e.to_string();
        diag_req_failure(call, "GET", &url, &format!("decode={msg}"));
        ApiError::Decode(msg)
    })?;
    let emails = raw
        .emails
        .unwrap_or_default()
        .into_iter()
        .map(|item| TripEmailItem {
            id: item
                .message_id
                .or(item.thread_id)
                .unwrap_or_else(|| "unknown".to_string()),
            subject: item.subject.unwrap_or_default(),
            sender: item.from.unwrap_or_default(),
            date: item.date.unwrap_or_default(),
            category: Category::Other,
            tags: item.tags.unwrap_or_default(),
        })
        .collect();

    Ok(TripEmailsResponse {
        trip_id: raw.trip_id.unwrap_or_else(|| trip_id.to_string()),
        emails,
    })
}
