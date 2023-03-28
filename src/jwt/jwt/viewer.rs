use yew::{classes, function_component, html, Html, Properties};

use super::Jwt;
use crate::utils::gen_copy_onclick;

#[derive(PartialEq, Eq, Properties)]
pub struct JwtViewerProps {
    pub jwt: Jwt,
}

#[function_component(JwtViewer)]
pub fn jwt_viewer(props: &JwtViewerProps) -> Html {
    let header = props.jwt.raw_header.clone();
    let payload = props.jwt.raw_payload.clone();
    let signature = props.jwt.raw_signature.clone();

    html! {
        <div>
            <span class={classes!("jwt-header")} onclick={gen_copy_onclick(header.clone())}>{header}</span>
            <span class={classes!("jwt-dot")}>{"."}</span>
            <span class={classes!("jwt-payload")} onclick={gen_copy_onclick(payload.clone())}>{payload}</span>
            <span class={classes!("jwt-dot")}>{"."}</span>
            <span class={classes!("jwt-signature")} onclick={gen_copy_onclick(signature.clone())}>{signature}</span>
        </div>
    }
}
