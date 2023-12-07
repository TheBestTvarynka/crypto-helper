use asn1_parser::{OwnedBitString, OwnedBmpString, OwnedOctetString, OwnedRawAsn1EntityData, OwnedUtf8String};
use yew::{function_component, html, Html, Properties};

use crate::asn1::node_options::NodeOptions;

#[derive(PartialEq, Properties, Clone)]
pub struct Utf8StringNodeProps {
    pub node: OwnedUtf8String,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(Utf8StringNode)]
pub fn utf8_string(props: &Utf8StringNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("UTF8String")}/>
            <span class="asn-simple-value">{props.node.string().to_owned()}</span>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct OctetStringNodeProps {
    pub node: OwnedOctetString,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(OctetStringNode)]
pub fn octet_string(props: &OctetStringNodeProps) -> Html {
    let octets = props.node.octets();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("Octet String")} />
            <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
            <span class="asn-simple-value">{hex::encode(octets)}</span>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct BitStringNodeProps {
    pub node: OwnedBitString,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(BitStringNode)]
pub fn bit_string(props: &BitStringNodeProps) -> Html {
    let bits = props.node.raw_bits()[1..]
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .fold(String::new(), |mut ac, new| {
            ac.push_str(&new);
            ac
        });
    let bits_amount = props.node.bits_amount();
    let bits = &bits[0..bits_amount];

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("Bit String")} />
            <span class="asn1-node-info-label">{format!("({} bits)", bits_amount)}</span>
            <span class="asn-simple-value">{bits}</span>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct BmpStringNodeProps {
    pub node: OwnedBmpString,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(BmpStringNode)]
pub fn bit_string(props: &BmpStringNodeProps) -> Html {
    let s = String::from_utf16_lossy(
        &props
            .node
            .raw_data()
            .chunks(2)
            .map(|bytes| u16::from_be_bytes(bytes.try_into().unwrap()))
            .collect::<Vec<_>>(),
    );

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("Bmp String")} />
            <span class="asn-simple-value">{s}</span>
        </div>
    }
}
