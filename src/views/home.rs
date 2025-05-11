use crate::components::Hero;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPlane;
use dioxus_free_icons::Icon;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Icon {
            width: 30,
            height: 30,
            fill: "black",
            icon: LdPlane,
        }
    }
}
