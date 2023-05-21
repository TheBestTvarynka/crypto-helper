use serde::__private::de::Content::String;
use web_sys::HtmlInputElement;
use yew::{Properties, Callback, Html, html, function_component, classes, TargetCast, use_state};

use crate::crypto_helper::algorithm::{BcryptAction, BcryptHashAction, BcryptInput as BI};
use crate::common::{Switch, build_byte_input};


#[derive(PartialEq, Properties, Clone)]
pub struct BcryptInputProps {
    pub input: BI,
    pub bcrypt_input_setter: Callback<BI>,
}

#[function_component(BcryptInput)]
pub fn bcrypt_input(input_props: &BcryptInputProps, ) -> Html {
    let data = input_props.input.data.clone();
    let bcrypt_action = input_props.input.action.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();

    let is_valid = use_state(|| true);

    let on_rounds_input = Callback::from(move |event: html::oninput::Event| {
        match event.target_unchecked_into::<HtmlInputElement>().value().parse::<u32>() {
            Ok(rounds) => {
                if let BcryptAction::Hash(bcrypt_hash_action) = bcrypt_action.clone() {
                    input_setter.emit(BI {
                        data: data.clone(),
                        action: BcryptAction::Hash(BcryptHashAction {
                            rounds,
                            salt: bcrypt_hash_action.salt,
                        })
                    })
                }
            },
            Err(_) => (),
        };
    });

    let is_valid = use_state(|| true);
    let set_is_valid = is_valid.setter();

    let input_setter = input_props.bcrypt_input_setter.clone();
    let bcrypt_action = input_props.input.action.clone();
    let data = input_props.input.data.clone();

    let on_salt_input = Callback::from(move |salt: Vec<u8>| {
        if let BcryptAction::Hash(hash_action) = bcrypt_action.clone() {
            let salt = match salt.len() {
                16 | 0 => {
                    set_is_valid.set(true);
                    salt
                },
                _ => {
                    set_is_valid.set(false);
                    salt
                }
            };
            input_setter.emit(BI {
                data: data.clone(),
                action: BcryptAction::Hash(BcryptHashAction {
                    salt,
                    rounds: hash_action.rounds,
                })
            })
        }
    });

    let bcrypt_action = input_props.input.action.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();
    let byte_setter = Callback::from(move |data| {
        input_setter.emit(BI {
            action: bcrypt_action.clone(),
            data,
        });
    });

    let input_setter = input_props.bcrypt_input_setter.clone();
    let data = input_props.input.data.clone();
    let input = input_props.input.clone();
    let on_switch = Callback::from(move |mode: bool| {
        let BI { action: _, data } = input.clone();
        input_setter.emit(BI {
            data,
            action: mode.into(),
        });
    });

    let data = input_props.input.data.clone();
    let input_setter = input_props.bcrypt_input_setter.clone();
    let bcrypt_hash_action = input_props.input.action.clone();
    let on_hashed_input = Callback::from(move |hashed: Vec<u8>| {
        if let BcryptAction::Verify(_) = bcrypt_hash_action.clone() {
            input_setter.emit(BI {
                data: data.clone(),
                action: BcryptAction::Verify(std::str::from_utf8(&hashed).unwrap_or("").to_string()),
            })
        }
    });

    let action = input_props.input.action.clone();
    let data = input_props.input.data.clone();
    html! {
        <div class={classes!("formats-container", "vertical")}>

            {build_byte_input(data, byte_setter, None, Some("bcrypt".into()))}

            <div class="horizontal">
                <span class="total">{"hash"}</span>
                <Switch id={"hash-verify".to_string()} setter={on_switch} state={<BcryptAction as Into<bool>>::into(action)}/>
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

pub fn build_bcrypt_input(
    input: BI,
    setter: Callback<BI>,
) -> Html {
    html! {
        <BcryptInput input={input} bcrypt_input_setter={setter}/>
    }
}