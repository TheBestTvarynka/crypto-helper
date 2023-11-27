use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::{crypto_helper::algorithm::{Argon2Input as Argon2InputData, Argon2Action}, common::{build_byte_input, BytesFormat, Switch}};

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
    let on_switch = Callback::from(move |mode: bool| {
        let Argon2InputData { data, ..} = input.clone();
        input_setter.emit(Argon2InputData {
            action: mode.into(),
            data
        })
    });

    let switch_state = bool::from(&props.input.action);
    html! {
        <div class={classes!("wide-input", "vertical")}>
            <div class="horizontal">
                <span class="total">{"hash"}</span>
                <Switch id={"switch".to_string()} setter={on_switch} state={switch_state}/> 
                <span class="total">{"verify"}</span>
            </div>
            {build_byte_input(data, setter, Some(BytesFormat::Ascii), Some("argon2".into()))}
        </div>
    }
}

pub fn build_argon2_input(input: Argon2InputData, setter: Callback<Argon2InputData>) -> Html {
    html! {
        <Argon2Input input={input} argon2_input_setter={setter}/>
    }
}
