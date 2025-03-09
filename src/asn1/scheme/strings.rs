use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::asn1::HighlightAction;
use crate::common::RcSlice;
use asn1_parser::{
    OwnedBitString, OwnedBmpString, OwnedGeneralString, OwnedIA5String, OwnedNumericString, OwnedOctetString,
    OwnedPrintableString, OwnedRawAsn1EntityData, OwnedUtf8String, OwnedVisibleString,
};
use std::fmt::Write;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct OctetStringNodeProps {
    pub node: OwnedOctetString,
    pub meta: OwnedRawAsn1EntityData,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(OctetStringNode)]
pub fn octet_string(props: &OctetStringNodeProps) -> Html {
    let octets = props.node.octets();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    match props.node.inner() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%;">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("OctetString")}/>
                    <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node)}
                </div>
            </div>
        },
        None => {
            let encoded_octets = match std::str::from_utf8(octets) {
                Ok(s) => {
                    if s.chars().any(|c| (c as u8) < 32) {
                        hex::encode(octets)
                    } else {
                        s.to_owned()
                    }
                }
                Err(_) => hex::encode(octets),
            };
            html! {
                <div class="terminal-asn1-node">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("OctetString")} />
                    <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                    <span class="asn-simple-value">{encoded_octets}</span>
                </div>
            }
        }
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct BitStringNodeProps {
    pub node: OwnedBitString,
    pub meta: OwnedRawAsn1EntityData,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(BitStringNode)]
pub fn bit_string(props: &BitStringNodeProps) -> Html {
    let raw_bits = props.node.raw_bits();
    let bits_amount = props.node.bits_amount();

    let bits = if raw_bits.len() > 1 {
        let mut bits = String::with_capacity((raw_bits.len() - 1) * 8);
        for byte in &raw_bits[1..] {
            write!(bits, "{:08b}", byte).unwrap();
        }
        bits
    } else {
        String::new()
    };

    let display_bits = if bits.len() >= bits_amount {
        &bits[0..bits_amount]
    } else {
        &bits[..]
    };

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    match props.node.inner() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%;">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("BitString")} />
                    <span class="asn1-node-info-label">{format!("({} bits)", bits_amount)}</span>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node)}
                </div>
            </div>
        },
        None => {
            html! {
                <div class="terminal-asn1-node">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("BitString")} />
                    <span class="asn1-node-info-label">{format!("({} bits)", bits_amount)}</span>
                    <span class="asn-simple-value">{display_bits}</span>
                </div>
            }
        }
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct BmpStringNodeProps {
    pub node: OwnedBmpString,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(BmpStringNode)]
pub fn bmp_string(props: &BmpStringNodeProps) -> Html {
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
            <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("BmpString")} />
            <span class="asn-simple-value">{s}</span>
        </div>
    }
}

define_string_node!(GeneralString);
define_string_node!(IA5String);
define_string_node!(PrintableString);
define_string_node!(Utf8String);
define_string_node!(NumericString);
define_string_node!(VisibleString);
