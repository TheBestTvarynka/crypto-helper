use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, function_component, html};

#[derive(PartialEq, Properties, Clone)]
pub struct NumberEditorProps {
    pub value: isize,
    pub setter: Callback<isize>,
    pub min: isize,
    pub max: isize,
}

#[function_component(NumberEditor)]
pub fn number_editor(props: &NumberEditorProps) -> Html {
    let setter = props.setter.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        if let Ok(value) = value.parse::<isize>() {
            setter.emit(value);
        }
    });

    html! {
        <input
            class={"modal-input"}
            cols={"5"}
            value={props.value.to_string()}
            {oninput}
            type="number"
            min={props.min.to_string()}
            max={props.max.to_string()}
        />
    }
}
