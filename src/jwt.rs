#[allow(clippy::module_inception)]
mod jwt;
pub mod jwt_utils;
mod jwte;
pub mod signature;
#[macro_use]
mod macros;

use std::str::FromStr;

use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::jwt::jwt::editor::JwtEditor;
use crate::jwt::jwt::viewer::JwtViewer;
use crate::jwt::jwt_utils::JwtUtils;
use crate::jwt::jwte::Jwte;

const TEST_JWT: &str = "eyJhbGciOiJIUzI1NiJ9.eyJSb2xlIjoiQWRtaW4iLCJJc3N1ZXIiOiJJc3N1ZXIiLCJVc2VybmFtZSI6IkphdmFJblVzZSIsImV4cCI6MTY3MDAwNDI1NCwiaWF0IjoxNjcwMDA0MjU0fQ.ZGsN42vr-bM4uxXowtlNl7xRerkdKu6i29VS8DFQ4Tw";

#[function_component(Jwt)]
pub fn jwt() -> Html {
    let notifications = use_notification::<Notification>();

    let raw_jwt = use_state(|| TEST_JWT.to_owned());
    let jwte = use_state(|| None);

    let jwt_setter = raw_jwt.setter();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        jwt_setter.set(input.value());
    });

    let raw = (*raw_jwt).clone();
    let jwte_setter = jwte.setter();
    let onclick = Callback::from(move |_| match Jwte::from_str(&raw) {
        Ok(jwte) => jwte_setter.set(Some(jwte)),
        Err(error) => {
            jwte_setter.set(None);
            log::error!("{}", error);

            notifications.spawn(Notification::new(
                NotificationType::Error,
                "Invalid token",
                error,
                Notification::NOTIFICATION_LIFETIME,
            ));
        }
    });

    let jwte_setter = jwte.setter();
    let set_jwt = Callback::from(move |jwt| {
        jwte_setter.set(Some(Jwte::Jwt(jwt)));
    });

    let jwte_setter = jwte.setter();

    html! {
        <article class={classes!("vertical")}>
            <textarea
                rows="5"
                placeholder={"base64 encoded JWT(JWE)"}
                class={classes!("base-input")}
                value={(*raw_jwt).clone()}
                {oninput}
            />
            <button {onclick}>{"Process"}</button>
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
