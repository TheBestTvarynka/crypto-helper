use yew::{Callback, Html, Properties, function_component, html};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SwitchProps {
    pub id: String,
    pub state: bool,
    pub setter: Callback<bool>,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps { state, setter, .. } = props.clone();

    let onchange = Callback::from(move |_| setter.emit(!state));

    html! {
        <span class="switch_span">
            <input type={"checkbox"} id={props.id.clone()} class="switch_input" {onchange} checked={props.state} />
            <label for={props.id.clone()} class="switch_label">{"Toggle"}</label>
        </span>
    }
}
