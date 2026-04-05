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
// Helpers
// ---------------------------------------------------------------------------

fn client() -> Result<(reqwest::Client, String), ApiError> {
    let cfg = APP_CONFIG.read();
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
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/trips"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    // Swagger currently models GET /trips as an inline object with
    // additionalProperties: array<tripResponse> (not a named TripsResponse).
    let raw: std::collections::HashMap<String, Vec<gen::TripResponse>> =
        handle_response(resp).await?;
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
    let (client, base) = client()?;
    let resp = client
        .post(format!("{base}/trips"))
        .json(&gen::CreateTripBody {
            name: Some(name.to_string()),
            date_range: Some(date_range.to_string()),
        })
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    // Swagger currently omits a concrete 200 response schema for POST /trips.
    // Backend returns a trip-like object; we decode as TripResponse by contract.
    let raw: gen::TripResponse = handle_response(resp).await?;
    Ok(map_trip(raw))
}

pub async fn get_trip_emails(trip_id: &str) -> Result<TripEmailsResponse, ApiError> {
    let (client, base) = client()?;
    let resp = client
        .get(format!("{base}/trips/{trip_id}/emails"))
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let raw: gen::TripEmailsResponse = handle_response(resp).await?;
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
        })
        .collect();

    Ok(TripEmailsResponse {
        trip_id: raw.trip_id.unwrap_or_else(|| trip_id.to_string()),
        emails,
    })
}
