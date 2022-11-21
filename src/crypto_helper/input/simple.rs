use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Debug, PartialEq, Properties)]
pub struct SimpleInputProps {
    pub input: String,
    pub setter: Callback<String>,
}

#[function_component(SimpleInput)]
pub fn simple_input(props: &SimpleInputProps) -> Html {
    let setter = props.setter.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        let value = input.value();
        log::debug!("simple input value: {}", value);

        setter.emit(value);
    });

    html! {
        <textarea
            rows="2"
            placeholder={"hex-encoded input"}
            class={classes!("base-input")}
            value={props.input.clone()}
            {oninput}
        />
    }
}

pub fn build_simple_input(input: String, setter: Callback<String>) -> Html {
    html! {
        <SimpleInput input={input} setter={setter} />
    }
}
