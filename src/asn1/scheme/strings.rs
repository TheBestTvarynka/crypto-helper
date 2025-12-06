use std::fmt::Write;

use asn1_parser::{
    Asn1Encoder, BitString, BmpString, GeneralString, IA5String, Mutable, NumericString, OctetString, PrintableString,
    RawAsn1EntityData, Utf8String, VisibleString,
};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::editor::{BYTES_FORMATS, IntegerEditor, StringEditor};
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

    let octets_node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: Vec<u8>| {
        octets_node.get_mut().set_octets(value);
        re_encode.emit(());
    });

    match node.inner() {
        Some(asn1) => {
            let asn1_type = asn1.inner_asn1().clone();
            let global_re_encode = props.re_encode.clone();
            let node = props.node.clone();
            let re_encode = Callback::from(move |_| {
                let mut buf = vec![0; asn1_type.needed_buf_size()];
                asn1_type.encode_buff(&mut buf).expect("Node encoding should not fail");

                node.get_mut().set_octets(buf);
                global_re_encode.emit(());
            });
            let add_node = Callback::from(move |_asn1_type| {
                // props.node.get_mut().set_inner(Some(Asn1::from_asn1_type(asn1_type)));
                // re_encode.emit(());
                // TODO
            });
            let remove_node = Callback::from(move |_| {
                // TODO
            });

            html! {
                <div style="cursor: crosshair; width: 100%;">
                    <div class="asn1-constructor-header">
                        <NodeOptions
                            node_bytes={RcSlice::from(props.meta.raw_bytes())}
                            {offset}
                            {length_len}
                            {data_len} name={String::from("OctetString")}
                            editor={Some(html! {
                                <IntegerEditor value={props.node.get().octets().to_vec()} {setter} formats={BYTES_FORMATS} />
                            })}
                        />
                        <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                    </div>
                    <div class="asn1-constructor-body">
                        {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node, re_encode, add_node, remove_node)}
                    </div>
                </div>
            }
        }
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
                    <NodeOptions
                        node_bytes={RcSlice::from(props.meta.raw_bytes())}
                        {offset}
                        {length_len}
                        {data_len}
                        name={String::from("OctetString")}
                        editor={Some(html! {
                            <IntegerEditor value={props.node.get().octets().to_vec()} {setter} formats={BYTES_FORMATS} />
                        })}
                    />
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

    let bits_node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: Vec<u8>| {
        bits_node.get_mut().set_bits(value);
        re_encode.emit(());
    });

    match node.inner() {
        Some(asn1) => {
            let asn1_type = asn1.inner_asn1().clone();
            let global_re_encode = props.re_encode.clone();
            let node = props.node.clone();
            let re_encode = Callback::from(move |_| {
                let mut buf = vec![0; asn1_type.needed_buf_size()];
                asn1_type.encode_buff(&mut buf).expect("Node encoding should not fail");

                node.get_mut().set_bits(buf);
                global_re_encode.emit(());
            });
            let add_node = Callback::from(move |_asn1_type| {
                // TODO
            });
            let remove_node = Callback::from(move |_| {
                // TODO
            });

            html! {
                <div style="cursor: crosshair; width: 100%;">
                    <div class="asn1-constructor-header">
                        <NodeOptions
                            node_bytes={RcSlice::from(props.meta.raw_bytes())}
                            {offset}
                            {length_len}
                            {data_len}
                            name={String::from("BitString")}
                            editor={Some(html! {
                                <IntegerEditor value={props.node.get().raw_bits().to_vec()} {setter} formats={BYTES_FORMATS} />
                            })}
                        />
                        <span class="asn1-node-info-label">{format!("({} bits)", bits_amount)}</span>
                    </div>
                    <div class="asn1-constructor-body">
                        {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node, re_encode, add_node, remove_node)}
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div class="terminal-asn1-node">
                    <NodeOptions
                        node_bytes={RcSlice::from(props.meta.raw_bytes())}
                        {offset}
                        {length_len}
                        {data_len}
                        name={String::from("BitString")}
                        editor={Some(html! {
                            <IntegerEditor value={props.node.get().raw_bits().to_vec()} {setter} formats={BYTES_FORMATS} />
                        })}
                    />
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
    pub re_encode: Callback<()>,
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

    let node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: String| {
        node.get_mut().set_string(&value);
        re_encode.emit(());
    });

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions
                node_bytes={RcSlice::from(props.meta.raw_bytes())}
                {offset}
                {length_len}
                {data_len}
                name={String::from("BmpString")}
                editor={Some(html! {
                    <StringEditor
                        value={s.clone()}
                        {setter}
                        validator={Callback::from(move |_| true)}
                    />
                })}
            />
            <span class="asn-simple-value">{s}</span>
        </div>
    }
}

define_string_node!(GeneralString, asn1_parser::validate_general);
define_string_node!(IA5String, asn1_parser::validate_ia5);
define_string_node!(PrintableString, asn1_parser::validate_printable);
define_string_node!(Utf8String, asn1_parser::validate_utf8);
define_string_node!(NumericString, asn1_parser::validate_numeric);
define_string_node!(VisibleString, asn1_parser::validate_visible);
