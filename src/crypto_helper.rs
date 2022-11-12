use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Timeout;
use picky_krb::crypto::CipherSuite;
use sha1::{Digest, Sha1};
use uuid::Uuid;
use yew::{classes, function_component, html, use_state, Callback, Html};

mod algorithm;
mod info;
mod input;
mod output;

use info::Info;
use input::Input;
use output::Output;

use crate::notification::{Notifications, Notification, NotificationType, get_new_notifications, NOTIFICATION_DURATION};

use self::algorithm::Algorithm;

fn from_hex(input: &str) -> Result<Vec<u8>, String> {
    hex::decode(input).map_err(|err| format!("invalid hex input:{:?}", err))
}

fn convert(algrithm: &Algorithm) -> Result<Vec<u8>, String> {
    match algrithm {
        Algorithm::Md5(input) => Ok(md5::compute(from_hex(input)?).to_vec()),
        Algorithm::Sha1(input) => {
            let mut sha1 = Sha1::new();
            sha1.update(from_hex(input)?);
            Ok(sha1.finalize().to_vec())
        }
        Algorithm::Sha256(input) => Ok(hmac_sha256::Hash::hash(&from_hex(input)?).to_vec()),
        Algorithm::Sha512(input) => Ok(hmac_sha512::Hash::hash(&from_hex(input)?).to_vec()),
        Algorithm::Aes128CtsHmacSha196(input) => {
            if input.mode {
                CipherSuite::Aes128CtsHmacSha196
                    .cipher()
                    .decrypt(
                        &from_hex(&input.key).map_err(|err| format!("key: {}", err))?,
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload).map_err(|err| format!("payload: {}", err))?,
                    )
                    .map_err(|err| err.to_string())
            } else {
                CipherSuite::Aes128CtsHmacSha196
                    .cipher()
                    .encrypt(
                        &from_hex(&input.key).map_err(|err| format!("key: {}", err))?,
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload).map_err(|err| format!("payload: {}", err))?,
                    )
                    .map_err(|err| err.to_string())
            }
        }
        Algorithm::Aes256CtsHmacSha196(input) => {
            if input.mode {
                CipherSuite::Aes256CtsHmacSha196
                    .cipher()
                    .decrypt(
                        &from_hex(&input.key).map_err(|err| format!("key: {}", err))?,
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload).map_err(|err| format!("payload: {}", err))?,
                    )
                    .map_err(|err| err.to_string())
            } else {
                CipherSuite::Aes256CtsHmacSha196
                    .cipher()
                    .encrypt(
                        &from_hex(&input.key).map_err(|err| format!("key: {}", err))?,
                        input.key_usage.parse::<i32>().unwrap_or_default(),
                        &from_hex(&input.payload).map_err(|err| format!("payload: {}", err))?,
                    )
                    .map_err(|err| err.to_string())
            }
        }
    }
}

#[function_component(CryptoHelper)]
pub fn crypto_helper() -> Html {
    let algorithm = use_state(Algorithm::default);
    let output = use_state(Vec::new);

    let notifications = use_state(|| Rc::new(RefCell::new(Vec::<Notification>::new())));
    let notification_to_delete = use_state(|| Option::None);

    let output_setter = output.setter();
    let notifications_setter = notifications.setter();
    let algorithm_data = (*algorithm).clone();
    let onclick_notifications = (*notifications).clone();
    let notification_to_delete_setter = notification_to_delete.setter();
    let onclick = Callback::from(move |_| {
        match convert(&algorithm_data) {
            Ok(output) => output_setter.set(output),
            Err(err) => {
                let id = Uuid::new_v4();
                let new_notificaion = Notification {
                    id,
                    notification_type: NotificationType::Error,
                    text: err,
                };

                let mut new_notifications = onclick_notifications.borrow().clone();
                new_notifications.push(new_notificaion);
                let new_notifications = RefCell::new(new_notifications);
                let timeout_notifications = new_notifications.clone();
                notifications_setter.set(new_notifications);

                let notifications_setter_timeout = notifications_setter.clone();
                let timeout = Timeout::new(NOTIFICATION_DURATION, move || {
                    log::debug!("in notification timeout handler: {:?}", id);
                    if let Some(notifications) = get_new_notifications(&id, &*timeout_notifications.borrow()) {
                        log::debug!("remove notification with id: {:?}", id);
                        notifications_setter_timeout.set(RefCell::new(notifications));
                    }
                });
                timeout.forget();
            },
        };
    });

    let notifications_setter = notifications.setter();
    let notifications_setter_callback = Callback::from(move |new_notifications| {
        // notifications_setter.set(RefCell::new(new_notifications));
    });

    html! {
        <article class={classes!("vertical")}>
            <Info set_algorithm={algorithm.setter()} algorithm={(*algorithm).clone()} />
            <Input algorithm={(*algorithm).clone()} setter={algorithm.setter()} />
            <div class={classes!("horizontal")}>
                <button onclick={onclick}>{"Go"}</button>
                <label for={"autoConvert"}>
                    <input type={"checkbox"} id={"autoConvert"} /><span>{"autogo"}</span>
                </label>
            </div>
            <Output algorithm={(*algorithm).clone()} output={(*output).clone()} />
            <Notifications notifications={(*notifications).clone()} setter={notifications_setter_callback} />
        </article>
    }
}
