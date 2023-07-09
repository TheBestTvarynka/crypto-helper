use asn1_parser::{OwnedUtf8String, OwnedOctetString};
use yew::{Html, html, function_component, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct Utf8StringNodeProps {
    pub node: OwnedUtf8String,
}

#[function_component(Utf8StringNode)]
pub fn utf8_string(props: &Utf8StringNodeProps) -> Html {
    html! {
        <div>
            <span>{"UTF8String"}</span>
            <span>{props.node.string().to_owned()}</span>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct OctetStringNodeProps {
    pub node: OwnedOctetString,
}

#[function_component(OctetStringNode)]
pub fn utf8_string(props: &OctetStringNodeProps) -> Html {
    let octets = props.node.octets();

    html! {
        <div>
            <span>{"Octet String"}</span>
            <span>{format!("({} bytes)", octets.len())}</span>
            <span>{hex::encode(octets)}</span>
        </div>
    }
}