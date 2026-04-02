use crate::components::bottom_nav::BottomNavBar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let route: Route = use_route();
    let active_tab = match route {
        Route::Home {} => "home",
        Route::EmailList {} => "emails",
        Route::Itinerary {} => "trips",
        Route::EmailDetail {} => "emails",
        Route::Settings {} => "settings",
    };

    rsx! {
        div { class: "flex flex-col min-h-screen",
            div { class: "flex-1 pb-16",
                Outlet::<Route> {}
            }
            BottomNavBar { active_tab: active_tab.to_string() }
        }
    }
}
