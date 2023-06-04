#[allow(clippy::module_inception)]
mod jwt;
pub mod jwt_utils;
mod jwte;
pub mod signature;
#[macro_use]
mod macros;

use std::str::FromStr;

pub use jwt::Jwt as JwtData;
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, TargetCast};
use yew_hooks::use_location;
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::common::Checkbox;
use crate::jwt::jwt::editor::JwtEditor;
use crate::jwt::jwt::viewer::JwtViewer;
use crate::jwt::jwt_utils::JwtUtils;
use crate::jwt::jwte::Jwte;
use crate::url_query_params;

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

    let notifications = use_notification::<Notification>();
    let oninput = if *auto_decode {
        let jwt_setter = raw_jwt.setter();
        let jwte_setter = jwte.setter();
        Callback::from(move |event: html::oninput::Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let value = input.value();

            jwt_setter.set(value.clone());

            match Jwte::from_str(&value) {
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
    use_effect_with_deps(
        move |_: &[(); 0]| {
            let query = &location.search;

            if query.len() < 2 {
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
        },
        [],
    );

    let jwte_setter = jwte.setter();
    let set_jwt = Callback::from(move |jwt| {
        jwte_setter.set(Some(Jwte::Jwt(jwt)));
    });

    let jwte_setter = jwte.setter();

    let set_auto_decode = auto_decode.setter();
    let set_checked = Callback::from(move |checked| {
        set_auto_decode.set(checked);
    });

    html! {
        <article class={classes!("vertical")}>
            <textarea
                rows="5"
                placeholder={"base64 encoded JWT(JWE)"}
                class={classes!("base-input")}
                value={(*raw_jwt).clone()}
                {oninput}
            />
            <div class={classes!("horizontal")}>
                <button {onclick}>{"Process"}</button>
                <Checkbox id={"auto-decode".to_owned()} name={"auto-decode".to_owned()} checked={*auto_decode} {set_checked} />
            </div>
            {if let Some(jwte) = &(*jwte) {
                match jwte {
                    Jwte::Jwt(jwt) => html! {
                        <div class={classes!("jwt-page")}>
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
                        <div class={classes!("container")}>
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
