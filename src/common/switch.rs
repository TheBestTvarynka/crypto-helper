use yew::{classes, function_component, html, Callback, Html, Properties, UseStateSetter};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SwitchProps {
    pub id: String,
    pub state: bool,
    pub setter: UseStateSetter<bool>,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps { state, setter, .. } = props.clone();

    let onchange = Callback::from(move |_| setter.set(!state));

    html! {
        <span class={classes!("switch_span")}>
            <input type={"checkbox"} id={props.id.clone()} class={classes!("switch_input")} {onchange} checked={props.state} />
            <label for={props.id.clone()} class={classes!("switch_label")}>{"Toggle"}</label>
        </span>
    }
}
