use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdHome, LdMail, LdMap, LdSettings};
use dioxus_free_icons::Icon;

#[component]
fn NavItem(
    id: &'static str,
    label: &'static str,
    active_tab: String,
    to: Route,
    children: Element,
) -> Element {
    let color_class = if active_tab == id {
        "text-primary"
    } else {
        "text-muted"
    };
    rsx! {
        Link { to: to, class: "flex flex-col items-center gap-1 {color_class} no-underline",
            {children}
            span { class: "text-xs", "{label}" }
        }
    }
}

#[component]
pub fn BottomNavBar(active_tab: String) -> Element {
    rsx! {
        div { class: "fixed bottom-0 left-0 right-0 flex justify-around items-center h-16 bg-card border-t border-border z-50",
            NavItem { id: "home", label: "Home", active_tab: active_tab.clone(), to: Route::Home {},
                Icon { icon: LdHome, width: 20, height: 20 }
            }
            NavItem { id: "emails", label: "Emails", active_tab: active_tab.clone(), to: Route::EmailList {},
                Icon { icon: LdMail, width: 20, height: 20 }
            }
            NavItem { id: "trips", label: "Trips", active_tab: active_tab.clone(), to: Route::Itinerary {},
                Icon { icon: LdMap, width: 20, height: 20 }
            }
            NavItem { id: "settings", label: "Settings", active_tab: active_tab.clone(), to: Route::Home {},
                Icon { icon: LdSettings, width: 20, height: 20 }
            }
        }
    }
}
