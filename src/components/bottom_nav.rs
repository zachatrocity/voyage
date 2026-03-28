use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdHome, LdMail, LdMap, LdSettings};
use dioxus_free_icons::Icon;

#[component]
pub fn BottomNavBar(active_tab: String) -> Element {
    let home_class = if active_tab == "home" { "text-primary" } else { "text-muted" };
    let emails_class = if active_tab == "emails" { "text-primary" } else { "text-muted" };
    let trips_class = if active_tab == "trips" { "text-primary" } else { "text-muted" };
    let settings_class = if active_tab == "settings" { "text-primary" } else { "text-muted" };

    rsx! {
        div { class: "fixed bottom-0 left-0 right-0 flex justify-around items-center h-16 bg-card border-t border-border z-50",
            Link {
                to: Route::Home {},
                class: "flex flex-col items-center gap-1 {home_class}",
                Icon { icon: LdHome, width: 20, height: 20 }
                span { class: "text-xs", "Home" }
            }
            Link {
                to: Route::EmailList {},
                class: "flex flex-col items-center gap-1 {emails_class}",
                Icon { icon: LdMail, width: 20, height: 20 }
                span { class: "text-xs", "Emails" }
            }
            Link {
                to: Route::Itinerary {},
                class: "flex flex-col items-center gap-1 {trips_class}",
                Icon { icon: LdMap, width: 20, height: 20 }
                span { class: "text-xs", "Trips" }
            }
            Link {
                to: Route::Home {},
                class: "flex flex-col items-center gap-1 {settings_class}",
                Icon { icon: LdSettings, width: 20, height: 20 }
                span { class: "text-xs", "Settings" }
            }
        }
    }
}
