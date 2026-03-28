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
