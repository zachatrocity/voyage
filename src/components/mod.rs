//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;
pub mod button;
pub mod card;
pub mod badge;
pub mod input;
pub mod bottom_sheet;
pub mod email_detail_card;
pub mod trip_chip;
