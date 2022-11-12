use time::OffsetDateTime;
use uuid::Uuid;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::utils::format_date_time;

// Indicates how much notification will live on the page: 5s
pub const NOTIFICATION_DURATION: u32 = 5_000;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NotificationType {
    Info,
    Warn,
    Error,
}

impl From<&NotificationType> for &str {
    fn from(notification_type: &NotificationType) -> Self {
        match notification_type {
            NotificationType::Info => "info",
            NotificationType::Warn => "warn",
            NotificationType::Error => "error",
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub notification_type: NotificationType,
    pub text: String,
}

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub notification: Notification,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(NotificationBadge)]
pub fn notification(props: &NotificationProps) -> Html {
    html! {
        <div class={classes!("notification", <&str>::from(&props.notification.notification_type))} onclick={props.onclick.clone()}>
            <span>{props.notification.text.clone()}</span>
            <span class={classes!("time")}>{OffsetDateTime::now_local().map(|now_time| format_date_time(&now_time)).unwrap_or_default()}</span>
        </div>
    }
}

pub fn get_new_notifications(
    id: &Uuid,
    notifications: &[Notification],
) -> Option<Vec<Notification>> {
    log::debug!("notifications len: {}", notifications.len());

    for (index, notification) in notifications.iter().enumerate() {
        if notification.id == *id {
            let mut new_notifications = notifications.to_vec();
            new_notifications.remove(index);

            return Some(new_notifications);
        }
    }

    None
}

#[derive(PartialEq, Properties)]
pub struct NotificationsProps {
    pub notifications: Vec<Notification>,
    pub delete_notification: Callback<Uuid>,
}

#[function_component(Notifications)]
pub fn notifications(props: &NotificationsProps) -> Html {
    let mut tn = Vec::new();
    for n in &*props.notifications {
        let notification_id = n.id;

        let delete_notification = props.delete_notification.clone();
        let onclick = Callback::from(move |_: MouseEvent| {
            delete_notification.emit(notification_id);
        });

        tn.push(html! {
            <NotificationBadge notification={n.clone()} onclick={onclick} />
        });
    }

    html! {
        <div class={classes!("notifications")}>
            {tn}
        </div>
    }
}
