use std::fmt::Debug;

use serde_json::{Value, to_string_pretty};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{Callback, Html, Properties, TargetCast, function_component, html, use_state};
use yew_hooks::use_clipboard;
use yew_notifications::{Notification, NotificationType, use_notification};

use super::Jwt;
use crate::common::{BytesFormat, Switch, TableView, build_simple_output};
use crate::utils::copy_to_clipboard_with_notification;

#[derive(PartialEq, Properties)]
pub struct JwtEditorProps {
    pub jwt: Jwt,
    pub set_jwt: Callback<Jwt>,
}

fn format_json<E: Debug>(
    value: impl Into<String>,
    name: impl Into<String>,
    set_data: Callback<String>,
    notify: Callback<Notification>,
    // Note: this function type can be changed (extended) in the future. It's just enough for now.
    format_fn: &'static dyn Fn(&serde_json::Value) -> Result<String, E>,
) -> Callback<MouseEvent> {
    let name = name.into();
    let value = value.into();

    Callback::from(move |_| {
        let value: Value = match serde_json::from_str(&value).map_err(|err| format!("{:?}", err)) {
            Ok(value) => value,
            Err(error) => {
                notify.emit(Notification::new(
                    NotificationType::Error,
                    name.clone(),
                    format!("Content is not a valid JSON: {}", error),
                    Notification::NOTIFICATION_LIFETIME,
                ));
                return;
            }
        };

        match format_fn(&value) {
            Ok(pretty_json_string) => set_data.emit(pretty_json_string),
            Err(error) => {
                notify.emit(Notification::new(
                    NotificationType::Error,
                    name.clone(),
                    format!("Can not prettify content: {:?}", error),
                    Notification::NOTIFICATION_LIFETIME,
                ));
            }
        }
    })
}

#[derive(Clone, Copy, Debug, Default)]
enum JsonView {
    Raw,
    #[default]
    Table,
}

impl From<bool> for JsonView {
    fn from(value: bool) -> Self {
        if value { JsonView::Table } else { JsonView::Raw }
    }
}

impl From<JsonView> for bool {
    fn from(value: JsonView) -> Self {
        match value {
            JsonView::Raw => false,
            JsonView::Table => true,
        }
    }
}

#[function_component(JwtEditor)]
pub fn jwt_editor(props: &JwtEditorProps) -> Html {
    let header = props.jwt.parsed_header.clone();
    let payload = props.jwt.parsed_payload.clone();
    let signature = props.jwt.parsed_signature.clone();
    let signature_bytes = props.jwt.signature.clone();

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let header_on_pretty = format_json(
        header.clone(),
        "Header",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.set_parsed_header(json);
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
        &to_string_pretty,
    );
    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let header_on_minify = format_json(
        header.clone(),
        "Header",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.set_parsed_header(json);
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
        &serde_json::to_string,
    );

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let payload_on_pretty = format_json(
        payload.clone(),
        "Payload",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_payload = json;
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
        &to_string_pretty,
    );
    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let notifications = use_notification::<Notification>();
    let payload_on_minify = format_json(
        payload.clone(),
        "Payload",
        Callback::from(move |json| {
            let mut jwt = jwt.clone();
            jwt.parsed_payload = json;
            set_jwt.emit(jwt);
        }),
        Callback::from(move |notification| notifications.spawn(notification)),
        &serde_json::to_string,
    );

    let set_jwt = props.set_jwt.clone();
    let jwt = props.jwt.clone();
    let on_header_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        let mut jwt = jwt.clone();
        jwt.set_parsed_header(value);
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

    let header_view = use_state(JsonView::default);
    let payload_view = use_state(JsonView::default);

    let header_view_setter = header_view.setter();
    let payload_view_setter = payload_view.setter();

    let notifications = use_notification::<Notification>();
    let clipboard = use_clipboard();

    html! {
        <div class="vertical">
            <div class="vertical">
                <div class="horizontal">
                    <span class="jwt-header" onclick={copy_to_clipboard_with_notification(header.clone(), clipboard.clone(), "Header", notifications.clone())}>{"Header"}</span>
                    <button onclick={header_on_pretty} class="jwt-util-button">{"Prettify"}</button>
                    <button onclick={header_on_minify} class="jwt-util-button">{"Minify"}</button>
                    <div class="horizontal">
                        <span class="total">{"raw"}</span>
                        <Switch id={String::from("jwt-header-view")} state={bool::from(*header_view)} setter={Callback::from(move |view: bool| header_view_setter.set(view.into()))} />
                        <span class="total">{"table"}</span>
                    </div>
                </div>
                {if !bool::from(*header_view) {html! {
                    <textarea rows="4" class="base-input" value={header} oninput={on_header_input} />
                }} else {html! {
                    <TableView value={serde_json::from_str::<Value>(&props.jwt.parsed_header).unwrap()} />
                }}}
            </div>
            <div class="vertical">
                <div class="horizontal">
                    <span class="jwt-payload" onclick={copy_to_clipboard_with_notification(payload.clone(), clipboard.clone(), "Payload", notifications.clone())}>{"Payload"}</span>
                    <button onclick={payload_on_pretty} class="jwt-util-button">{"Prettify"}</button>
                    <button onclick={payload_on_minify} class="jwt-util-button">{"Minify"}</button>
                    <div class="horizontal">
                        <span class="total">{"raw"}</span>
                        <Switch id={String::from("jwt-payload-view")} state={bool::from(*payload_view)} setter={Callback::from(move |view: bool| payload_view_setter.set(view.into()))} />
                        <span class="total">{"table"}</span>
                    </div>
                </div>
                {if !bool::from(*payload_view) {html! {
                    <textarea rows="6" class="base-input" value={payload} oninput={on_payload_input} />
                }} else {html! {
                    <TableView value={serde_json::from_str::<Value>(&props.jwt.parsed_payload).unwrap()} />
                }}}
            </div>
            <div class="vertical">
                <span class="jwt-signature" onclick={copy_to_clipboard_with_notification(signature.clone(), clipboard, "Signature", notifications.clone())}>{"Signature"}</span>
                {build_simple_output(signature_bytes, BytesFormat::Hex, Callback::from(move |notification| notifications.spawn(notification)))}
            </div>
        </div>
    }
}
