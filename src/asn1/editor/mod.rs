use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, function_component, html};

#[derive(PartialEq, Properties, Clone)]
pub struct StringEditorProps {
    pub value: String,
    pub setter: Callback<String>,
}

#[function_component(StringEditor)]
pub fn string_editor(props: &StringEditorProps) -> Html {
    let setter = props.setter.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        setter.emit(value);
    });

    html! {
        <div>
            <textarea
                class={"base-input"}
                value={props.value.clone()}
                {oninput}
            />
        </div>
    }
}
