//! The views module contains the components for all Layouts and Routes for our app.

mod home;
pub use home::Home;

mod navbar;
pub use navbar::Navbar;

mod itinerary;
pub use itinerary::Itinerary;

mod email_detail;
pub use email_detail::EmailDetail;

mod email_list;
pub use email_list::EmailList;
