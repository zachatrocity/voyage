use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdCircleCheck, LdClock};
use dioxus_free_icons::Icon;

use crate::types::ItineraryStatus;

#[component]
pub fn StatusBadge(status: ItineraryStatus) -> Element {
    match status {
        ItineraryStatus::Confirmed => rsx! {
            span { class: "text-xs bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400 rounded-full px-2 py-0.5 flex items-center gap-1 whitespace-nowrap",
                Icon { icon: LdCircleCheck, width: 12, height: 12 }
                "Confirmed"
            }
        },
        ItineraryStatus::Pending => rsx! {
            span { class: "text-xs bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400 rounded-full px-2 py-0.5 flex items-center gap-1 whitespace-nowrap",
                Icon { icon: LdClock, width: 12, height: 12 }
                "Pending"
            }
        },
    }
}
