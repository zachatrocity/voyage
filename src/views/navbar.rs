use crate::Route;
use dioxus::prelude::*;

/// Layout wrapper — renders child routes via Outlet.
/// Navigation is handled by BottomNavBar in each view.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
