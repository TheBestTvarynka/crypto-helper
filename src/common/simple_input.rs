use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Debug, PartialEq, Properties)]
pub struct SimpleInputProps {
    pub input: String,
    pub placeholder: String,
    pub setter: Callback<String>,
}

#[function_component(SimpleInput)]
pub fn simple_input(props: &SimpleInputProps) -> Html {
    let setter = props.setter.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        setter.emit(value);
    });

    html! {
        <div>
            <textarea
                rows="2"
                placeholder={props.placeholder.clone()}
                class={classes!("base-input")}
                value={props.input.clone()}
                {oninput}
            />
            <span class={classes!("total")}>{"total bytes: "}{props.input.len() / 2}</span>
        </div>
    }
}

pub fn build_simple_input(input: String, placeholder: String, setter: Callback<String>) -> Html {
    html! {
        <SimpleInput {input} {setter} {placeholder} />
    }
}
