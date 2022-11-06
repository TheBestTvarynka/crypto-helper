mod simple;

use yew::{html, Html, Properties, function_component};

use super::Algorithm;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct OutputProps {
    pub algorithm: Algorithm,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    html! {
        <div>
            {format!("output: {:?}", props)}
        </div>
    }
}
