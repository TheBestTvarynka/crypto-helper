use time::OffsetDateTime;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

use crate::utils::format_date_time;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Notification {
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

#[function_component(Notifications)]
pub fn notifications() -> Html {
    let notifications = use_state(|| {
        vec![
            Notification {
                notification_type: NotificationType::Info,
                text: "Info notification".into(),
            },
            Notification {
                notification_type: NotificationType::Warn,
                text: "Warn notification".into(),
            },
            Notification {
                notification_type: NotificationType::Error,
                text: "Error notification".into(),
            },
        ]
    });

    let mut tn = Vec::with_capacity((*notifications).len());
    for (i, n) in (*notifications).iter().enumerate() {
        let setter = notifications.setter();

        let mut new_notifications = (*notifications).clone();
        new_notifications.remove(i);

        let onclick = Callback::from(move |_: MouseEvent| {
            // it's needed clone because yew requires this closure to implement Fn (but not FnOnce)
            setter.set(new_notifications.clone());
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
