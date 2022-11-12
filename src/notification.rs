use std::{cell::RefCell, rc::Rc};

use time::OffsetDateTime;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html, Properties};
use uuid::Uuid;

use crate::utils::format_date_time;

// Indicates how much notification will live on the page: 3s
pub const NOTIFICATION_DURATION: u32 = 3_000;

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

pub fn get_new_notifications(id: &Uuid, notifications: &[Notification]) -> Option<Vec<Notification>> {
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

fn get_index_to_remove(id: &Uuid, notifications: &[Notification]) -> Option<usize> {
    for (index, notification) in notifications.iter().enumerate() {
        if notification.id == *id {
            return Some(index);
        }
    }

    None
}

#[derive(PartialEq, Properties)]
pub struct NotificationsProps {
    pub notifications: Rc<RefCell<Vec<Notification>>>,
    pub setter: Callback<Vec<Notification>>,
}

#[function_component(Notifications)]
pub fn notifications(props: &NotificationsProps) -> Html {
    let mut tn = Vec::new();
    for n in &*props.notifications.borrow() {
        let notification_id = n.id;

        // let timeout_notifications = (*notifications).clone();
        // let setter = notifications.setter();
        // let timeout = Timeout::new(5_000, move || {
        //     if let Some(notifications) = new_notifications(&notification_id, &timeout_notifications) {
        //         setter.set(notifications);
        //     }
        // });
        // timeout.forget();
        
        let onclick_notifications = props.notifications.clone();
        // let setter = props.setter.clone();
        let onclick = Callback::from(move |_: MouseEvent| {
            let index = get_index_to_remove(&notification_id, &*onclick_notifications.borrow());

            log::debug!("remove: {:?}", index);
            if index.is_none() {
                return;
            }

            let mut ns = onclick_notifications.borrow_mut();
            ns.remove(index.unwrap());
            
            // if let Some(notifications) = get_new_notifications(&notification_id, &onclick_notifications) {
            //     setter.emit(notifications);
            // }
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
