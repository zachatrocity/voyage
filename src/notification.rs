use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NotificationKind {
    Error,
    Success,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct AppNotification {
    pub message: String,
    pub kind: NotificationKind,
}

pub static NOTIFICATION: GlobalSignal<Option<AppNotification>> = Signal::global(|| None);

pub fn notify_error(msg: impl Into<String>) {
    *NOTIFICATION.write() = Some(AppNotification {
        message: msg.into(),
        kind: NotificationKind::Error,
    });
}

pub fn notify_success(msg: impl Into<String>) {
    *NOTIFICATION.write() = Some(AppNotification {
        message: msg.into(),
        kind: NotificationKind::Success,
    });
}

pub fn notify_info(msg: impl Into<String>) {
    *NOTIFICATION.write() = Some(AppNotification {
        message: msg.into(),
        kind: NotificationKind::Info,
    });
}
