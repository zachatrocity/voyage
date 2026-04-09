use dioxus::prelude::*;

use crate::types::{Category, Email};

fn format_email_datetime(raw: &str) -> String {
    let trimmed = raw.trim();
    let (date_part, time_part) = match trimmed.split_once('T') {
        Some((d, t)) => (d, Some(t)),
        None => return trimmed.to_string(),
    };

    let mut date_parts = date_part.split('-');
    let (year, month, day) = match (date_parts.next(), date_parts.next(), date_parts.next()) {
        (Some(y), Some(m), Some(d)) => (y, m, d.trim_start_matches('0')),
        _ => return trimmed.to_string(),
    };

    let month_name = match month {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return trimmed.to_string(),
    };

    let formatted_date = format!("{month_name} {}, {year}", if day.is_empty() { "0" } else { day });

    let Some(tp) = time_part else {
        return formatted_date;
    };

    let time_token = tp
        .split(['Z', '+', '-'])
        .next()
        .unwrap_or("")
        .trim();
    let mut hm = time_token.split(':');
    let (hour_str, minute_str) = match (hm.next(), hm.next()) {
        (Some(h), Some(m)) => (h, m),
        _ => return formatted_date,
    };

    let hour24 = match hour_str.parse::<u32>() {
        Ok(h) if h < 24 => h,
        _ => return formatted_date,
    };
    let minute = match minute_str.parse::<u32>() {
        Ok(m) if m < 60 => m,
        _ => return formatted_date,
    };

    let (hour12, meridiem) = match hour24 {
        0 => (12, "AM"),
        1..=11 => (hour24, "AM"),
        12 => (12, "PM"),
        _ => (hour24 - 12, "PM"),
    };

    format!("{formatted_date} at {hour12}:{minute:02} {meridiem}")
}

#[component]
pub fn EmailDetailCard(
    email: Email,
    full_body: Option<String>,
    full_html: Option<String>,
    loading_full_body: bool,
) -> Element {
    let body_text = full_body.unwrap_or_else(|| email.body_preview.clone());
    let formatted_date = format_email_datetime(&email.date);

    let emoji = match email.category {
        Category::Flight => "✈️",
        Category::Hotel => "🏨",
        Category::CarRental => "🚗",
        Category::Cruise => "🚢",
        Category::Activity => "🎡",
        Category::Other => "📧",
    };

    rsx! {
        div { class: "bg-card rounded-xl shadow-sm mx-4 mt-4 p-4",
            // Sender row
            div { class: "flex items-center gap-3 mb-3",
                div { class: "w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-lg shrink-0",
                    "{emoji}"
                }
                div { class: "flex-1 min-w-0",
                    p { class: "text-sm font-semibold text-foreground truncate", "{email.sender}" }
                    p { class: "text-xs text-muted truncate", "{email.sender_email}" }
                }
                span { class: "text-xs text-muted shrink-0", "{formatted_date}" }
            }

            // Subject
            h2 { class: "text-base font-bold text-foreground mb-2", "{email.subject}" }

            // Category badge
            span { class: "text-xs border border-primary/30 text-primary rounded-full px-2 py-0.5 inline-block mb-3",
                "{email.category.label()}"
            }

            // Body
            div { class: "max-h-[52vh] overflow-y-auto pr-1",
                if loading_full_body {
                    p { class: "text-xs text-muted mb-2", "Loading full email…" }
                }

                if let Some(html) = full_html {
                    div {
                        class: "text-sm text-foreground leading-relaxed break-words [&_a]:text-primary [&_a]:underline",
                        dangerous_inner_html: "{html}",
                    }
                } else {
                    p { class: "text-sm text-muted leading-relaxed whitespace-pre-wrap break-words",
                        "{body_text}"
                    }
                }
            }
        }
    }
}
