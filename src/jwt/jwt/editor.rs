use serde_json::{to_string_pretty, Value};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use super::Jwt;
use crate::utils::gen_copy_onclick;

#[derive(PartialEq, Properties)]
pub struct JwtEditorProps {
    pub jwt: Jwt,
    pub set_jwt: Callback<Jwt>,
}

fn get_onclick_prettify(
    value: impl Into<String>,
    name: impl Into<String>,
    set_data: Callback<String>,
    notify: Callback<Notification>,
) -> Callback<MouseEvent> {
    let name = name.into();
    let value = value.into();

    Callback::from(move |_| {
        let value: Value = match serde_json::from_str(&value).map_err(|err| format!("{:?}", err)) {
            Ok(value) => value,
            Err(error) => {
                log::error!("{:?}", error);
                notify.emit(Notification::new(
                    NotificationType::Error,
                    name.clone(),
                    format!("Content is not a valid JSON: {:?}", error),
                ));
                return;
            }
        };

        match to_string_pretty(&value) {
            Ok(pretty_json_string) => set_data.emit(pretty_json_string),
            Err(error) => {
                log::error!("{:?}", error);
                notify.emit(Notification::new(
                    NotificationType::Error,
                    name.clone(),
                    format!("Can not prettify content: {:?}", error),
                ));
            }
        }
    })
}

#[function_component(JwtEditor)]
pub fn jwt_editor(props: &JwtEditorProps) -> Html {
    let header = props.jwt.parsed_header.clone();
    let payload = props.jwt.parsed_payload.clone();
    let signature = props.jwt.parsed_signature.clone();

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let header_on_pretty = get_onclick_prettify(
        header.clone(),
        "Header",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_header = json;
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
    );

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let payload_on_pretty = get_onclick_prettify(
        payload.clone(),
        "Payload",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_payload = json;
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
    );

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let on_header_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        let mut jwt = jwt.clone();
        jwt.parsed_header = value;
        set_jwt.emit(jwt);
    });

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let on_payload_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        let mut jwt = jwt.clone();
        jwt.parsed_payload = value;
        set_jwt.emit(jwt);
    });

    html! {
        <div class={classes!("vertical")}>
            <div class={classes!("vertical")}>
                <div class={classes!("horizontal")}>
                    <span class={classes!("jwt-header")} onclick={gen_copy_onclick(header.clone())}>{"Header"}</span>
                    <button onclick={header_on_pretty} class={classes!("jwt-util-button")}>{"Prettify"}</button>
                </div>
                <textarea rows="4" class={classes!("base-input")} value={header} oninput={on_header_input} />
            </div>
            <div class={classes!("vertical")}>
                <div class={classes!("horizontal")}>
                    <span class={classes!("jwt-payload")} onclick={gen_copy_onclick(payload.clone())}>{"Payload"}</span>
                    <button onclick={payload_on_pretty} class={classes!("jwt-util-button")}>{"Prettify"}</button>
                </div>
                <textarea rows="6" class={classes!("base-input")} value={payload} oninput={on_payload_input} />
            </div>
            <div class={classes!("vertical")}>
                <span class={classes!("jwt-signature")} onclick={gen_copy_onclick(signature.clone())}>{"Signature"}</span>
                <textarea rows="2" class={classes!("base-input")} value={signature} readonly={true} />
            </div>
        </div>
    }
}
