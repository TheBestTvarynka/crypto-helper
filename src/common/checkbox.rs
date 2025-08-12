use yew::{Callback, Html, Properties, function_component, html};

#[derive(PartialEq, Properties, Clone)]
pub struct CheckboxProps {
    pub id: String,
    pub name: String,
    pub checked: bool,
    pub set_checked: Callback<bool>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        id,
        name,
        checked,
        set_checked,
    } = props.clone();

    let oninput = Callback::from(move |_| set_checked.emit(!checked));

    html! {
        <div class="checkbox">
            <input type="checkbox" checked={props.checked} id={id.clone()} {oninput} />
            <label for={id.clone()} class="checkbox-label" />
            <label for={id}>{name}</label>
        </div>
    }
}
