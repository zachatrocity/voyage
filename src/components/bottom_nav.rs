use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdHome, LdMail, LdMap, LdSettings};
use dioxus_free_icons::Icon;

#[component]
fn NavItem(id: &'static str, label: &'static str, active_tab: String, children: Element) -> Element {
    let color_class = if active_tab == id { "text-primary" } else { "text-muted" };
    rsx! {
        div { class: "flex flex-col items-center gap-1 {color_class}",
            {children}
            span { class: "text-xs", "{label}" }
        }
    }
}

#[component]
pub fn BottomNavBar(active_tab: String) -> Element {
    rsx! {
        div { class: "fixed bottom-0 left-0 right-0 flex justify-around items-center h-16 bg-card border-t border-border",
            NavItem { id: "home", label: "Home", active_tab: active_tab.clone(),
                Icon { icon: LdHome, width: 20, height: 20 }
            }
            NavItem { id: "emails", label: "Emails", active_tab: active_tab.clone(),
                Icon { icon: LdMail, width: 20, height: 20 }
            }
            NavItem { id: "trips", label: "Trips", active_tab: active_tab.clone(),
                Icon { icon: LdMap, width: 20, height: 20 }
            }
            NavItem { id: "settings", label: "Settings", active_tab: active_tab.clone(),
                Icon { icon: LdSettings, width: 20, height: 20 }
            }
        }
    }
}
