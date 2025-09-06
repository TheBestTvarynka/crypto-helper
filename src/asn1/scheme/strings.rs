use std::fmt::Write;

use asn1_parser::{
    BitString, BmpString, GeneralString, IA5String, Mutable, NumericString, OctetString, PrintableString,
    RawAsn1EntityData, Utf8String, VisibleString,
};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct OctetStringNodeProps {
    pub node: Mutable<OctetString>,
    pub meta: RawAsn1EntityData,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub re_encode: Callback<()>,
}

#[function_component(OctetStringNode)]
pub fn octet_string(props: &OctetStringNodeProps) -> Html {
    let node = props.node.get();
    let octets = node.octets();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    match node.inner() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%;">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("OctetString")}/>
                    <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node, props.re_encode.clone())}
                </div>
            </div>
        },
        None => {
            let encoded_octets = if octets.len() % 2 == 0
                && let Ok(s) = String::from_utf16(&{
                    octets
                        .chunks(2)
                        .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
                        .collect::<Vec<_>>()
                }) {
                s.to_owned()
            } else if let Ok(s) = std::str::from_utf8(octets) {
                s.to_owned()
            } else {
                hex::encode(octets)
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
    pub node: Mutable<BitString>,
    pub meta: RawAsn1EntityData,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub re_encode: Callback<()>,
}

#[function_component(BitStringNode)]
pub fn bit_string(props: &BitStringNodeProps) -> Html {
    let node = props.node.get();
    let raw_bits = node.raw_bits();
    let bits_amount = node.bits_amount();

    let bits = if raw_bits.len() > 1 {
        let mut bits = String::with_capacity((raw_bits.len() - 1) * 8);
        for byte in &raw_bits[1..] {
            write!(bits, "{:08b}", byte).unwrap();
        }
        bits
    } else {
        String::new()
    };

    let display_bits = &bits[0..bits_amount.min(bits.len())];

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    match node.inner() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%;">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("BitString")} />
                    <span class="asn1-node-info-label">{format!("({} bits)", bits_amount)}</span>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node, props.re_encode.clone())}
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
    pub node: Mutable<BmpString>,
    pub meta: RawAsn1EntityData,
}

#[function_component(BmpStringNode)]
pub fn bmp_string(props: &BmpStringNodeProps) -> Html {
    let s = String::from_utf16_lossy(
        &props
            .node
            .get()
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
