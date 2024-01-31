use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::{
    common::{build_byte_input, BytesFormat, Switch},
    crypto_helper::algorithm::{Argon2Action, Argon2Input as Argon2InputData, Argon2Variant, Argon2Version},
};

#[derive(PartialEq, Properties, Clone)]
pub struct Argon2InputProps {
    pub input: Argon2InputData,
    pub argon2_input_setter: Callback<Argon2InputData>,
}

#[function_component(Argon2Input)]
pub fn argon2_input(props: &Argon2InputProps) -> Html {
    let data: Vec<u8> = props.input.data.clone();

    let input_setter = props.argon2_input_setter.clone();
    let action: Argon2Action = props.input.action.clone();
    let setter = Callback::from(move |data| {
        input_setter.emit(Argon2InputData {
            action: action.clone(),
            data,
        })
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
    let input = props.input.clone();
    let get_hash_verify_switch_state = Callback::from(move |()| -> bool { (&input.action).into() });

    let hash_verify_switch_state = bool::from(&props.input.action);

    let input_setter = props.argon2_input_setter.clone();
    let input = props.input.clone();
    let set_version = move |event: web_sys::Event| {
        let target = event.target().expect("NO TARGET");
        let version: String = js_sys::Reflect::get(target.as_ref(), &"value".into())
            .unwrap()
            .as_string()
            .unwrap();

        let input = input.clone();
        let input = input.set_version(version.as_str().try_into().unwrap());
        log::debug!("{input:?}");
        input_setter.emit(input.clone());
    };
    html! {
        <div class={classes!("wide-input", "vertical")}>
            <div class="horizontal">
                <span class="total">{"hash"}</span>
                <Switch id={"switch".to_string()} setter={on_hash_verify_switch} state={hash_verify_switch_state}/>
                <span class="total">{"verify"}</span>
            </div>
            {build_byte_input(data, setter, Some(BytesFormat::Ascii), Some("argon2".into()))}

            {
                if !get_hash_verify_switch_state.emit(()) {
                    html! {
                        <>
                            {"Version"}
                            <select onchange={set_version}>
                                <option value="Argon10">{"Argon10"}</option>
                                <option value="Argon13">{"Argon13"}</option>
                            </select>
                        </>
                    }
                } else {
                    html! {

                    }
                }
            }
        </div>
    }
}

pub fn build_argon2_input(input: Argon2InputData, setter: Callback<Argon2InputData>) -> Html {
    html! {
        <Argon2Input input={input} argon2_input_setter={setter}/>
    }
}
