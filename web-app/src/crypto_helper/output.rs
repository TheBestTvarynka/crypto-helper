mod simple;

use yew::{function_component, html, Html, Properties};

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
