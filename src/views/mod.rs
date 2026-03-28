//! The views module contains the components for all Layouts and Routes for our app.

mod home;
pub use home::Home;

mod blog;
pub use blog::Blog;

mod navbar;
pub use navbar::Navbar;

mod itinerary;
pub use itinerary::Itinerary;

mod email_list;
pub use email_list::EmailList;
