use asn1_parser::Bool;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct BoolNodeProps {
    pub node: Bool,
}

#[function_component(BoolNode)]
pub fn bool(props: &BoolNodeProps) -> Html {
    html! {
        <div class="terminal-asn1-node">
            <span>{"Bool"}</span>
            <span>{props.node.value()}</span>
        </div>
    }
}

#[function_component(NullNode)]
pub fn null() -> Html {
    html! {
        <div class="terminal-asn1-node">
            <span>{"Null"}</span>
        </div>
    }
}
