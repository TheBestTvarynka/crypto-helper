use yew::{classes, function_component, html, Html, Properties};
use yew_hooks::use_clipboard;

use super::Jwt;
use crate::utils::get_copy_to_clipboard_callback;

#[derive(PartialEq, Eq, Properties)]
pub struct JwtViewerProps {
    pub jwt: Jwt,
}

#[function_component(JwtViewer)]
pub fn jwt_viewer(props: &JwtViewerProps) -> Html {
    let header = props.jwt.raw_header.clone();
    let payload = props.jwt.raw_payload.clone();
    let signature = props.jwt.raw_signature.clone();
    let rest = props.jwt.rest.clone();

    let clipboard = use_clipboard();

    html! {
        <div>
            <span class={classes!("jwt-header")} onclick={get_copy_to_clipboard_callback(header.clone(), clipboard.clone())}>{header}</span>
            <span class={classes!("jwt-dot")}>{"."}</span>
            <span class={classes!("jwt-payload")} onclick={get_copy_to_clipboard_callback(payload.clone(), clipboard.clone())}>{payload}</span>
            <span class={classes!("jwt-dot")}>{"."}</span>
            <span class={classes!("jwt-signature")} onclick={get_copy_to_clipboard_callback(signature.clone(), clipboard.clone())}>{signature}</span>
            {if !rest.is_empty() {
                html! {
                    <span class={classes!("jwt-rest")} onclick={get_copy_to_clipboard_callback(rest.clone(), clipboard)}>{rest}</span>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
