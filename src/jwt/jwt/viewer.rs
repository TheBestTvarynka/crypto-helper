use yew::{function_component, html, Html, Properties};
use yew_hooks::use_clipboard;
use yew_notifications::{use_notification, Notification};

use super::Jwt;
use crate::utils::copy_to_clipboard_with_notification;

#[derive(PartialEq, Eq, Properties)]
pub struct JwtViewerProps {
    pub jwt: Jwt,
}

#[function_component(JwtViewer)]
pub fn jwt_viewer(props: &JwtViewerProps) -> Html {
    let header = props.jwt.raw_header.clone();
    let payload = props.jwt.raw_payload.clone();
    let signature = props.jwt.raw_signature.clone();
    let start_over = props.jwt.start_over.clone();
    let leftover = props.jwt.leftover.clone();

    let clipboard = use_clipboard();
    let notifications = use_notification::<Notification>();

    html! {
        <div>
            <span class="jwt-rest" onclick={copy_to_clipboard_with_notification(start_over.clone(), clipboard.clone(), "Start part", notifications.clone())}>{start_over}</span>
            <span class="jwt-header" onclick={copy_to_clipboard_with_notification(header.clone(), clipboard.clone(), "Header", notifications.clone())}>{header}</span>
            <span class="jwt-dot">{"."}</span>
            <span class="jwt-payload" onclick={copy_to_clipboard_with_notification(payload.clone(), clipboard.clone(), "Payload", notifications.clone())}>{payload}</span>
            <span class="jwt-dot">{"."}</span>
            <span class="jwt-signature" onclick={copy_to_clipboard_with_notification(signature.clone(), clipboard.clone(), "Signature", notifications.clone())}>{signature}</span>
            {if !leftover.is_empty() {
                html! {
                    <span class="jwt-rest" onclick={copy_to_clipboard_with_notification(leftover.clone(), clipboard, "Token leftover", notifications)}>{leftover}</span>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
