use yew::{html, Html, Properties, function_component};

use super::Algorithm;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct InputProps {
    pub algorithm: Algorithm,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div>
            {format!("input: {:?}", props)}
        </div>
    }
}
