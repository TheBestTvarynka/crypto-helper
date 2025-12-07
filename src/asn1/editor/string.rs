use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, function_component, html, use_node_ref, use_state};

#[derive(PartialEq, Properties, Clone)]
pub struct StringEditorProps {
    pub value: String,
    pub setter: Callback<String>,
    #[prop_or_default]
    pub rows: Option<usize>,
    pub validator: Callback<String, bool>,
}

#[function_component(StringEditor)]
pub fn string_editor(props: &StringEditorProps) -> Html {
    let is_valid = use_state(|| true);

    let input_ref = use_node_ref();

    let setter = props.setter.clone();
    let validator = props.validator.clone();
    let is_valid_setter = is_valid.setter();
    let input_node = input_ref.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        let is_valid = validator.emit(value.clone());
        is_valid_setter.set(is_valid);

        if !is_valid && let Some(el) = input_node.cast::<web_sys::HtmlInputElement>() {
            let class_list = el.class_list();
            if class_list.contains("modal-input-blink")
                && let Err(err) = el.class_list().remove_1("modal-input-blink")
            {
                warn!(?err, "Failed to remove class from modal input");
            }

            let _ = el.offset_height();

            if let Err(err) = class_list.add_1("modal-input-blink") {
                warn!(?err, "Failed to add class to modal input");
            }

            let _ = el.offset_height();
        }

        if is_valid {
            setter.emit(value);
        }
    });

    let cols = props.value.find('\n').unwrap_or(props.value.len()) + 10;

    html! {
        <textarea
            ref={input_ref}
            class={"modal-input"}
            cols={cols.to_string()}
            value={props.value.clone()}
            {oninput}
            rows={props.rows.unwrap_or(props.value.lines().count()).to_string()}
        />
    }
}
