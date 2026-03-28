use dioxus::prelude::*;

use components::Hero;
use views::{Blog, EmailDetail, Home, Navbar};

mod components;
mod types;
mod views;

use types::*;

pub static EMAILS: GlobalSignal<Vec<Email>> = Signal::global(|| seed_emails());
pub static TRIPS: GlobalSignal<Vec<Trip>> = Signal::global(|| seed_trips());
pub static SELECTED_EMAIL: GlobalSignal<Option<String>> = Signal::global(|| None);
pub static SELECTED_TRIP: GlobalSignal<Option<String>> = Signal::global(|| None);

fn seed_emails() -> Vec<Email> {
    vec![
        Email {
            id: "e1".into(),
            subject: "Flight Confirmation: DL-442 Boise to Orlando".into(),
            sender: "Delta Air Lines".into(),
            sender_email: "noreply@delta.com".into(),
            date: "Mar 15, 2026".into(),
            body_preview: "Thank you for booking with Delta. Your confirmation number is XKRT72. Flight DL-442 departs Boise (BOI) on June 14 at 8:00 AM...".into(),
            category: Category::Flight,
            trip_id: Some("t1".into()),
        },
        Email {
            id: "e2".into(),
            subject: "Your Reservation at Disney's Polynesian Resort".into(),
            sender: "Disney Resort".into(),
            sender_email: "reservations@disney.com".into(),
            date: "Mar 16, 2026".into(),
            body_preview: "Your reservation is confirmed. Check-in: June 14, 2026. Check-out: June 18, 2026. Confirmation: WDW-88432...".into(),
            category: Category::Hotel,
            trip_id: Some("t1".into()),
        },
        Email {
            id: "e3".into(),
            subject: "Carnival Cruise Booking Confirmation — Horizon".into(),
            sender: "Carnival Cruise Line".into(),
            sender_email: "noreply@carnival.com".into(),
            date: "Feb 10, 2026".into(),
            body_preview: "Thank you for booking Carnival Horizon. Embarkation: Port Canaveral, June 18, 2026 at 12:00 PM. Booking ID: CCL-99201...".into(),
            category: Category::Cruise,
            trip_id: Some("t2".into()),
        },
        Email {
            id: "e4".into(),
            subject: "Your United Airlines eTicket Receipt".into(),
            sender: "United Airlines".into(),
            sender_email: "noreply@united.com".into(),
            date: "Mar 20, 2026".into(),
            body_preview: "E-ticket receipt for UA-2241 Chicago to Boise. Confirmation: UA-XK881...".into(),
            category: Category::Flight,
            trip_id: None,
        },
        Email {
            id: "e5".into(),
            subject: "Marriott Bonvoy: Reservation Confirmed".into(),
            sender: "Marriott Hotels".into(),
            sender_email: "noreply@marriott.com".into(),
            date: "Mar 21, 2026".into(),
            body_preview: "Your stay at Chicago Marriott Downtown is confirmed. Check-in: Aug 5, 2026. Confirmation: MRRT-2091...".into(),
            category: Category::Hotel,
            trip_id: None,
        },
    ]
}

fn seed_trips() -> Vec<Trip> {
    vec![
        Trip {
            id: "t1".into(),
            name: "2026 Disney Family Trip".into(),
            date_range: "Jun 14 – Jun 18, 2026".into(),
            email_count: 2,
            confirmed_count: 2,
        },
        Trip {
            id: "t2".into(),
            name: "2026 Caribbean Cruise".into(),
            date_range: "Jun 18 – Jun 25, 2026".into(),
            email_count: 1,
            confirmed_count: 1,
        },
    ]
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
    #[end_layout]
    #[route("/email")]
    EmailDetail {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
