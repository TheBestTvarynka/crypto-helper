use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::common::{build_byte_input, BytesFormat, Switch};
use crate::crypto_helper::algorithm::{BcryptAction, BcryptHashAction, BcryptInput as BcryptInputData};

#[derive(PartialEq, Properties, Clone)]
pub struct BcryptInputProps {
    pub input: BcryptInputData,
    pub bcrypt_input_setter: Callback<BcryptInputData>,
}

#[function_component(BcryptInput)]
pub fn bcrypt_input(input_props: &BcryptInputProps) -> Html {
    let notifications = use_notification::<Notification>();

    let data = input_props.input.data.clone();
    let bcrypt_action = input_props.input.action.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();

    let on_rounds_input = Callback::from(move |event: html::oninput::Event| {
        match event.target_unchecked_into::<HtmlInputElement>().value().parse::<u32>() {
            Ok(rounds) => {
                if let BcryptAction::Hash(bcrypt_hash_action) = bcrypt_action.clone() {
                    input_setter.emit(BcryptInputData {
                        data: data.clone(),
                        action: BcryptAction::Hash(BcryptHashAction {
                            rounds,
                            salt: bcrypt_hash_action.salt,
                        }),
                    });
                }
            }
            Err(err) => notifications.spawn(Notification::new(
                NotificationType::Error,
                "Processing error",
                err.to_string(),
                Notification::NOTIFICATION_LIFETIME,
            )),
        };
    });

    let input_setter = input_props.bcrypt_input_setter.clone();
    let bcrypt_action = input_props.input.action.clone();
    let data = input_props.input.data.clone();

    let on_salt_input = Callback::from(move |salt: Vec<u8>| {
        if let BcryptAction::Hash(hash_action) = bcrypt_action.clone() {
            input_setter.emit(BcryptInputData {
                data: data.clone(),
                action: BcryptAction::Hash(BcryptHashAction {
                    salt,
                    rounds: hash_action.rounds,
                }),
            })
        }
    });

    let bcrypt_action = input_props.input.action.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();
    let byte_setter = Callback::from(move |data| {
        input_setter.emit(BcryptInputData {
            action: bcrypt_action.clone(),
            data,
        });
    });

    let input_setter = input_props.bcrypt_input_setter.clone();
    let input = input_props.input.clone();
    let on_switch = Callback::from(move |mode: bool| {
        let BcryptInputData { action: _, data } = input.clone();
        input_setter.emit(BcryptInputData {
            data,
            action: mode.into(),
        });
    });

    let notifications = use_notification::<Notification>();

    let data = input_props.input.data.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();
    let bcrypt_hash_action = input_props.input.action.clone();

    let on_hashed_input = Callback::from(move |hashed: Vec<u8>| {
        if let BcryptAction::Verify(_) = bcrypt_hash_action.clone() {
            input_setter.emit(BcryptInputData {
                data: data.clone(),
                action: BcryptAction::Verify(
                    std::str::from_utf8(&hashed)
                        .unwrap_or_else(|err| {
                            notifications.spawn(Notification::new(
                                NotificationType::Error,
                                "Invalid hash",
                                err.to_string(),
                                Notification::NOTIFICATION_LIFETIME,
                            ));
                            ""
                        })
                        .to_string(),
                ),
            })
        }
    });

    let action = input_props.input.action.clone();
    let data = input_props.input.data.clone();
    html! {
        <div class={classes!("formats-container", "vertical")}>
            {build_byte_input(data, byte_setter, Some(BytesFormat::Ascii), Some("bcrypt".into()))}
            <div class="horizontal">
                <span class="total">{"hash"}</span>
                <Switch id={"hash-verify".to_string()} setter={on_switch} state={bool::from(action)}/>
                <span class="total">{"verify"}</span>
            </div>
            {match input_props.input.action.clone() {
                BcryptAction::Hash(hash_info) => html! {
                    <div class="horizontal">
                        <input class={classes!("base-input")} value={hash_info.rounds.to_string()} type="number" min="4" max="31" placeholder={"rounds"} oninput={on_rounds_input}/>
                        {build_byte_input(hash_info.salt, on_salt_input, None, Some("salt".into()))}
                    </div>
                },
                BcryptAction::Verify(hashed) => html! {
                    {build_byte_input(hashed.into_bytes(), on_hashed_input, None, Some("hashed".into()))}
                },
            }}
        </div>
    }
}

pub fn build_bcrypt_input(input: BcryptInputData, setter: Callback<BcryptInputData>) -> Html {
    html! {
        <BcryptInput input={input} bcrypt_input_setter={setter}/>
    }
}
