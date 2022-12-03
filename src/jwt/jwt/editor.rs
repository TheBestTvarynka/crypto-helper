use serde_json::{to_string_pretty, Value};
use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::utils::gen_copy_onclick;

use super::Jwt;

#[derive(PartialEq, Properties)]
pub struct JwtEditorProps {
    pub jwt: Jwt,
    pub set_jwt: Callback<Jwt>,
}

fn get_onclick_prettify(json: Value, set_data: Callback<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| match to_string_pretty(&json) {
        Ok(pretty_json_string) => set_data.emit(pretty_json_string),
        Err(error) => log::error!("{:?}", error),
    })
}

#[function_component(JwtEditor)]
pub fn jwt_editor(props: &JwtEditorProps) -> Html {
    let header = props.jwt.parsed_header.clone();
    let payload = props.jwt.parsed_payload.clone();
    let signature = props.jwt.parsed_signature.clone();

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let header_on_pretty = get_onclick_prettify(
        props.jwt.header.clone(),
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_header = json;
            set_jwt.emit(jwt);
        }),
    );

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let payload_on_pretty = get_onclick_prettify(
        props.jwt.payload.clone(),
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_payload = json;
            set_jwt.emit(jwt);
        }),
    );

    html! {
        <div class={classes!("vertical")}>
            <div class={classes!("vertical")}>
                <div class={classes!("horizontal")}>
                    <span class={classes!("jwt-header")} onclick={gen_copy_onclick(header.clone())}>{"Header"}</span>
                    <button onclick={header_on_pretty} class={classes!("jwt-util-button")}>{"Prettify"}</button>
                </div>
                <textarea rows="4" class={classes!("base-input")} value={header} />
            </div>
            <div class={classes!("vertical")}>
                <div class={classes!("horizontal")}>
                    <span class={classes!("jwt-payload")} onclick={gen_copy_onclick(payload.clone())}>{"Payload"}</span>
                    <button onclick={payload_on_pretty} class={classes!("jwt-util-button")}>{"Prettify"}</button>
                </div>
                <textarea rows="6" class={classes!("base-input")} value={payload} />
            </div>
            <div class={classes!("vertical")}>
                <span class={classes!("jwt-signature")} onclick={gen_copy_onclick(signature.clone())}>{"Signature"}</span>
                <textarea rows="2" class={classes!("base-input")} value={signature} />
                // <BytesViewer bytes={props.jwt.signature.clone()} />
            </div>
        </div>
    }
}
