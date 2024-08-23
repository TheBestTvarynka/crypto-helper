#[allow(clippy::module_inception)]
mod jwt;
pub mod jwt_utils;
mod jwte;
pub mod signature;
#[macro_use]
mod macros;

use std::str::FromStr;

use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, TargetCast};
use yew_hooks::{use_local_storage, use_location};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::common::Checkbox;
use crate::jwt::jwt::editor::JwtEditor;
use crate::jwt::jwt::viewer::JwtViewer;
use crate::jwt::jwt_utils::JwtUtils;
use crate::jwt::jwte::Jwte;
use crate::url_query_params;

const JWT_LOCAL_STORAGE_KEY: &str = "JWT_DATA";
const TEST_JWT: &str = "eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw";

#[function_component(Jwt)]
pub fn jwt() -> Html {
    let raw_jwt = use_state(|| TEST_JWT.to_owned());
    let jwte = use_state(|| None);
    let auto_decode = use_state(|| true);

    let raw = (*raw_jwt).clone();
    let jwte_setter = jwte.setter();
    let notifications = use_notification::<Notification>();
    let onclick = Callback::from(move |_| match Jwte::from_str(&raw) {
        Ok(jwte) => jwte_setter.set(Some(jwte)),
        Err(error) => {
            jwte_setter.set(None);

            notifications.spawn(Notification::new(
                NotificationType::Error,
                "Invalid token",
                error,
                Notification::NOTIFICATION_LIFETIME,
            ));
        }
    });

    let jwt_setter = raw_jwt.setter();
    let jwte_setter = jwte.setter();
    let notifications = use_notification::<Notification>();
    let parse_jwt_callback = Callback::from(move |raw_jwt: String| {
        jwt_setter.set(raw_jwt.clone());

        match Jwte::from_str(&raw_jwt) {
            Ok(jwte) => jwte_setter.set(Some(jwte)),
            Err(error) => {
                jwte_setter.set(None);

                notifications.spawn(Notification::new(
                    NotificationType::Error,
                    "Invalid token",
                    error,
                    Notification::NOTIFICATION_LIFETIME,
                ));
            }
        };
    });

    let oninput = if *auto_decode {
        let parse_jwt_callback = parse_jwt_callback.clone();
        Callback::from(move |event: html::oninput::Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            parse_jwt_callback.emit(input.value());
        })
    } else {
        let jwt_setter = raw_jwt.setter();
        Callback::from(move |event: html::oninput::Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let value = input.value();

            jwt_setter.set(value);
        })
    };

    let location = use_location();
    let jwt_setter = raw_jwt.setter();
    let jwte_setter = jwte.setter();
    let notifications = use_notification::<Notification>();
    let local_storage = use_local_storage::<String>(JWT_LOCAL_STORAGE_KEY.to_owned());
    use_effect_with([], move |_: &[(); 0]| {
        let query = &location.search;

        if query.len() < 2 {
            // URL query params is empty. We try to load JWT from local storage.
            if let Some(raw_jwt) = (*local_storage).as_ref() {
                match serde_json::from_str(raw_jwt.as_str()) {
                    Ok(jwt) => {
                        jwte_setter.set(Some(Jwte::Jwt(jwt)));
                    }
                    Err(err) => {
                        error!(?err, "Can not load JWT from local storage.");
                    }
                }
            }
            return;
        }

        let jwt: url_query_params::Jwt = match serde_qs::from_str(&query[1..]) {
            Ok(jwt) => jwt,
            Err(err) => {
                notifications.spawn(Notification::new(
                    NotificationType::Error,
                    "Can not load data from url",
                    err.to_string(),
                    Notification::NOTIFICATION_LIFETIME,
                ));
                return;
            }
        };

        jwt_setter.set(jwt.jwt.clone());
        match Jwte::from_str(&jwt.jwt) {
            Ok(jwte) => jwte_setter.set(Some(jwte)),
            Err(error) => {
                jwte_setter.set(None);

                notifications.spawn(Notification::new(
                    NotificationType::Error,
                    "Invalid token",
                    error,
                    Notification::NOTIFICATION_LIFETIME,
                ));
            }
        };
    });

    let local_storage = use_local_storage::<String>(JWT_LOCAL_STORAGE_KEY.to_owned());
    use_effect_with(jwte.clone(), move |jwte| {
        let jwte: &Option<Jwte> = jwte;
        if let Some(Jwte::Jwt(jwt)) = jwte {
            local_storage.set(serde_json::to_string(jwt).expect("JWT serialization should not fail"));
        }
    });

    let jwte_setter = jwte.setter();
    let set_jwt = Callback::from(move |jwt| {
        jwte_setter.set(Some(Jwte::Jwt(jwt)));
    });

    let jwte_setter = jwte.setter();

    let set_auto_decode = auto_decode.setter();
    let set_checked = Callback::from(move |checked| {
        set_auto_decode.set(checked);
    });

    let raw_jwt_data = (*raw_jwt).clone();
    let onkeydown = Callback::from(move |event: KeyboardEvent| {
        if event.ctrl_key() && event.code() == "Enter" {
            parse_jwt_callback.emit(raw_jwt_data.clone())
        }
    });

    html! {
        <article class="vertical">
            <textarea
                rows="5"
                placeholder={"base64 encoded JWT(JWE)"}
                class="base-input"
                value={(*raw_jwt).clone()}
                {oninput}
                {onkeydown}
            />
            <div class="horizontal">
                <button class="action-button" {onclick}>{"Process"}</button>
                <Checkbox id={"auto-decode".to_owned()} name={"auto-decode".to_owned()} checked={*auto_decode} {set_checked} />
            </div>
            {if let Some(jwte) = &(*jwte) {
                match jwte {
                    Jwte::Jwt(jwt) => html! {
                        <div class="jwt-page">
                            <JwtViewer jwt={jwt.clone()} />
                            <JwtEditor jwt={jwt.clone()} {set_jwt} />
                        </div>
                    },
                    Jwte::Jwe(_jwe) => html! {},
            }} else {
                html! {}
            }}
            {if let Some(jwte) = &(*jwte) {
                match jwte {
                    Jwte::Jwt(jwt) => html! {
                        <div class="container">
                            <JwtUtils jwt={jwt.clone()} set_jwt={Callback::from(move |jwt| jwte_setter.set(Some(Jwte::Jwt(jwt))))} />
                        </div>
                    },
                    Jwte::Jwe(_jwe) => html! {},
            }} else {
                html! {}
            }}
        </article>
    }
}
