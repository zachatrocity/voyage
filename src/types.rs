use crate::voyage_api::types::NotmuchEmailResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Flight,
    Hotel,
    CarRental,
    Cruise,
    Activity,
    Other,
}

impl Category {
    pub fn label(&self) -> &str {
        match self {
            Category::Flight => "✈️ Flight",
            Category::Hotel => "🏨 Hotel",
            Category::CarRental => "🚗 Car Rental",
            Category::Cruise => "🚢 Cruise",
            Category::Activity => "🎡 Activity",
            Category::Other => "📧 Other",
        }
    }
    pub fn color_class(&self) -> &str {
        match self {
            Category::Flight => "text-accent",
            Category::Hotel => "text-cta",
            Category::CarRental => "text-muted",
            Category::Cruise => "text-primary",
            Category::Activity => "text-accent",
            Category::Other => "text-muted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItineraryStatus {
    Confirmed,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub subject: String,
    pub sender: String,
    pub sender_email: String,
    pub date: String,
    pub body_preview: String,
    pub category: Category,
    pub trip_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trip {
    pub id: String,
    pub name: String,
    pub date_range: String,
    pub email_count: usize,
    pub confirmed_count: usize,
}

impl From<NotmuchEmailResult> for Email {
    fn from(r: NotmuchEmailResult) -> Self {
        let from_str = r.from.unwrap_or_default();
        // Backend "from" field is "Name <email@example.com>" — split it
        let (sender, sender_email) = if let Some(start) = from_str.find('<') {
            let name = from_str[..start].trim().to_string();
            let email = from_str[start + 1..].trim_end_matches('>').to_string();
            (if name.is_empty() { email.clone() } else { name }, email)
        } else {
            (from_str.clone(), from_str.clone())
        };

        let category = match r.category.as_deref().unwrap_or("other") {
            "flight" => Category::Flight,
            "hotel" => Category::Hotel,
            "car_rental" => Category::CarRental,
            "cruise" => Category::Cruise,
            "activity" => Category::Activity,
            _ => Category::Other,
        };

        Self {
            id: r.message_id.unwrap_or_default(),
            subject: r.subject.unwrap_or_default(),
            sender,
            sender_email,
            date: r.date.unwrap_or_default(),
            body_preview: r.body_preview.unwrap_or_default(),
            category,
            trip_id: r.trip_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItineraryItem {
    pub id: String,
    pub trip_id: String,
    pub email_id: String,
    pub title: String,
    pub detail: String,
    pub sub_detail: Option<String>,
    pub date: String,
    pub category: Category,
    pub status: ItineraryStatus,
}
