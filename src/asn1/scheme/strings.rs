use asn1_parser::{OwnedBitString, OwnedOctetString, OwnedUtf8String};
use yew::{function_component, html, Html, Properties};

use crate::common::BytesViewer;

#[derive(PartialEq, Properties, Clone)]
pub struct Utf8StringNodeProps {
    pub node: OwnedUtf8String,
}

#[function_component(Utf8StringNode)]
pub fn utf8_string(props: &Utf8StringNodeProps) -> Html {
    html! {
        <div class="terminal-asn1-node">
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
pub fn octet_string(props: &OctetStringNodeProps) -> Html {
    let octets = props.node.octets();

    html! {
        <div class="terminal-asn1-node">
            <span>{"Octet String"}</span>
            <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
            // <span>{hex::encode(octets)}</span>
            <BytesViewer bytes={octets.to_vec()} />
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct BitStringNodeProps {
    pub node: OwnedBitString,
}

#[function_component(BitStringNode)]
pub fn bit_string(props: &BitStringNodeProps) -> Html {
    let octets = props.node.bits();

    html! {
        <div class="terminal-asn1-node">
            <span>{"Bit String"}</span>
            <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
            // <span>{hex::encode(octets)}</span>
            <BytesViewer bytes={octets.to_vec()} />
        </div>
    }
}
