use web_sys::HtmlInputElement;
use yew::html::TargetCast;
use yew::{Callback, Html, Properties, classes, function_component, html};

use crate::common::{BytesFormat, Switch, build_byte_input};
use crate::crypto_helper::algorithm::{Argon2Action, Argon2HashAction, Argon2Input as Argon2InputData};

#[derive(PartialEq, Properties, Clone)]
pub struct Argon2InputProps {
    pub input: Argon2InputData,
    pub argon2_input_setter: Callback<Argon2InputData>,
}

#[function_component(Argon2Input)]
pub fn argon2_input(props: &Argon2InputProps) -> Html {
    let input_setter = props.argon2_input_setter.clone();
    let action: Argon2Action = props.input.action.clone();
    let setter = Callback::from(move |data| {
        input_setter.emit(Argon2InputData {
            action: action.clone(),
            data,
        })
    });

    let data: Vec<u8> = props.input.data.clone();
    let input_setter = props.argon2_input_setter.clone();
    let action: Argon2Action = props.input.action.clone();

    let salt_setter = Callback::from(move |salt| {
        if let Argon2Action::Hash(hash_action) = action.clone() {
            input_setter.emit(Argon2InputData {
                action: Argon2Action::Hash(Argon2HashAction { salt, ..hash_action }),
                data: data.clone(),
            })
        }
    });

    let data: Vec<u8> = props.input.data.clone();
    let input_setter = props.argon2_input_setter.clone();
    let action: Argon2Action = props.input.action.clone();

    let hash_setter = Callback::from(move |hash: Vec<u8>| {
        if let Argon2Action::Verify(_) = action.clone() {
            input_setter.emit(Argon2InputData {
                action: Argon2Action::Verify(hash),
                data: data.clone(),
            })
        }
    });

    let input_setter = props.argon2_input_setter.clone();
    let input = props.input.clone();
    let on_hash_verify_switch = Callback::from(move |mode: bool| {
        let Argon2InputData { data, .. } = input.clone();
        input_setter.emit(Argon2InputData {
            action: mode.into(),
            data,
        })
    });
    let hash_verify_switch_state = bool::from(&props.input.action);

    let input_setter = props.argon2_input_setter.clone();
    let input = props.input.clone();

    let set_version = move |event: yew::html::onchange::Event| {
        let html_element: HtmlInputElement = event.target_unchecked_into();
        let input = input.with_version(html_element.value().as_str().try_into().unwrap());
        input_setter.emit(input.clone());
    };

    let input_setter = props.argon2_input_setter.clone();
    let input = props.input.clone();

    let set_variant = move |event: yew::html::onchange::Event| {
        let html_element: HtmlInputElement = event.target_unchecked_into();
        let input = input.with_variant(html_element.value().as_str().try_into().unwrap());
        input_setter.emit(input.clone());
    };

    let data: Vec<u8> = props.input.data.clone();
    html! {
        <div class={classes!("wide-input", "vertical")}>
            <div class="horizontal">
                <span class="total">{"hash"}</span>
                <Switch id={"switch".to_string()} setter={on_hash_verify_switch} state={hash_verify_switch_state}/>
                <span class="total">{"verify"}</span>
            </div>
            {build_byte_input(data, setter, Some(BytesFormat::Ascii), Some("password".into()))}
            {match &props.input.action {
                Argon2Action::Hash(hash_action) => html! {
                    <div class="vertical">
                        {build_byte_input(hash_action.salt.clone(), salt_setter, Some(BytesFormat::Ascii), Some("salt".into()))}
                        <div class="horizontal">
                            <select onchange={set_version} class="base-input">
                                <option value="Argon10">{"Argon10"}</option>
                                <option value="Argon13">{"Argon13"}</option>
                            </select>
                            <select onchange={set_variant} class="base-input">
                                <option value="Argon2i">{"Argon2i"}</option>
                                <option value="Argon2d">{"Argon2d"}</option>
                                <option value="Argon2id">{"Argon2id"}</option>
                            </select>
                        </div>
                    </div>
                },
                Argon2Action::Verify(hash) => html! {
                    <>
                        {build_byte_input(hash.clone(), hash_setter, Some(BytesFormat::Ascii), Some("hash".into()))}
                    </>
                }}}
        </div>
    }
}

pub fn build_argon2_input(input: Argon2InputData, setter: Callback<Argon2InputData>) -> Html {
    html! {
        <Argon2Input input={input} argon2_input_setter={setter}/>
    }
}
