use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationType};

use crate::{crypto_helper::algorithm::Argon2Input as Argon2InputData, common::{build_byte_input, BytesFormat}};

#[derive(PartialEq, Properties, Clone)]
pub struct Argon2InputProps {
    pub input: Argon2InputData,
    pub argon2_input_setter: Callback<Argon2InputData>,
}

#[function_component(Argon2Input)]
pub fn argon2_input(props: &Argon2InputProps) -> Html {
    let data = props.input.data.clone();
    let input_setter = props.argon2_input_setter.clone();
    let action = props.input.action.clone();
    let setter = Callback::from(move |data| {
        input_setter.emit(Argon2InputData {
            action: action.clone(),
            data,
        })
    });
    html! {
        <div class={classes!("wide-input", "vertical")}>
            {build_byte_input(data, setter, Some(BytesFormat::Ascii), Some("argon2".into()))}
        </div>
    }
}

pub fn build_argon2_input(input: Argon2InputData, setter: Callback<Argon2InputData>) -> Html {
    html! {
        <Argon2Input input={input} argon2_input_setter={setter}/>
    }
}
