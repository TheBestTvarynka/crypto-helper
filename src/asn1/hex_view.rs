use asn1_parser::{Asn1, Asn1Entity, Asn1Type, OwnedAsn1, RawAsn1EntityData};
use web_sys::MouseEvent;
use yew::virtual_dom::VNode;
use yew::{classes, function_component, html, Callback, Classes, Html, Properties};

use crate::asn1::node_options::NodeOptions;
use crate::asn1::{compare_ids, HighlightAction};

#[derive(PartialEq, Properties, Clone)]
pub struct HexViewerProps {
    pub structure: OwnedAsn1,

    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(HexViewer)]
pub fn hex_viewer(props: &HexViewerProps) -> Html {
    html! {
        <div class="asn1-hex-viewer">
            <div class="asn1-hex-node">
                {{
                    let set_cur_node = props.set_cur_node.clone();
                    let mut bytes = Vec::with_capacity(props.structure.meta().raw_data.len());
                    build_hex_bytes(&props.structure, &props.cur_node.clone(), set_cur_node, &mut bytes, false);
                    bytes
                }}
            </div>
        </div>
    }
}

fn format_bytes(
    meta: &RawAsn1EntityData,
    bytes: &[u8],
    asn1_node_id: u64,
    class: Classes,
    set_cur_node: Callback<HighlightAction>,
    formatted_bytes: &mut Vec<VNode>,
) {
    let offset = meta.tag_position();
    let length_len = meta.length_range().len();
    let data_len = meta.data_range().len();

    bytes.iter().for_each(|byte| {
        let set_cur_node_enter = set_cur_node.clone();
        let onmouseenter = Callback::from(move |_: MouseEvent| set_cur_node_enter.emit(HighlightAction::Show(asn1_node_id)));
        let set_cur_node = set_cur_node.clone();
        let onmouseleave = Callback::from(move |_: MouseEvent| set_cur_node.emit(HighlightAction::Hide(asn1_node_id)));
        formatted_bytes.push(html! {
            <span class={classes!("asn1-hex-byte", class.clone())} {onmouseenter} {onmouseleave}>
                <NodeOptions node_bytes={meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("{:02x?}", byte)}/>
            </span>
        })
    });
}

fn build_hex_bytes(
    asn1: &Asn1<'_>,
    cur_node: &Option<u64>,
    set_cur_node: Callback<HighlightAction>,
    bytes: &mut Vec<VNode>,
    select_all: bool,
) {
    let asn1_node_id = asn1.id();
    let if_selected = compare_ids(asn1_node_id, cur_node);

    let tag: u8 = asn1.tag().into();
    let tag_set_cur_node = set_cur_node.clone();
    let onmouseenter = Callback::from(move |_: MouseEvent| tag_set_cur_node.emit(HighlightAction::Show(asn1_node_id)));
    let tag_set_cur_node = set_cur_node.clone();
    let onmouseleave = Callback::from(move |_: MouseEvent| tag_set_cur_node.emit(HighlightAction::Hide(asn1_node_id)));

    let meta = asn1.meta();

    let offset = meta.tag_position();
    let length_len = meta.length_range().len();
    let data_len = meta.data_range().len();

    bytes.push(html! {
        <span
            class={if select_all {
                "asn1-hex-byte asn1-hex-byte-data-selected"
            } else if if_selected {
                "asn1-hex-byte asn1-hex-byte-tag-selected"
            } else {
                "asn1-hex-byte asn1-hex-byte-tag"
            }}
            {onmouseenter}
            {onmouseleave}
        >
            <NodeOptions node_bytes={meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("{:02x?}", tag)}/>
        </span>
    });

    format_bytes(
        meta,
        asn1.meta().length_bytes(),
        asn1_node_id,
        if select_all {
            classes!("asn1-hex-byte-data-selected")
        } else if if_selected {
            classes!("asn1-hex-byte-len-selected")
        } else {
            classes!("asn1-hex-byte-len")
        },
        set_cur_node.clone(),
        bytes,
    );

    build_data_bytes(
        asn1,
        asn1_node_id,
        cur_node,
        set_cur_node,
        bytes,
        if_selected || select_all,
    );
}

fn build_data_bytes(
    asn1: &Asn1<'_>,
    asn1_node_id: u64,
    cur_node: &Option<u64>,
    set_cur_node: Callback<HighlightAction>,
    bytes: &mut Vec<VNode>,
    select_all: bool,
) {
    fn default_bytes(
        asn1_node_id: u64,
        cur_node: &Option<u64>,
        set_cur_node: Callback<HighlightAction>,
        asn1: &Asn1<'_>,
        bytes: &mut Vec<VNode>,
        select_all: bool,
    ) {
        let if_selected = compare_ids(asn1_node_id, cur_node);

        format_bytes(
            asn1.meta(),
            asn1.meta().data_bytes(),
            asn1_node_id,
            if if_selected || select_all {
                classes!("asn1-hex-byte-data-selected")
            } else {
                classes!("asn1-hex-byte-data")
            },
            set_cur_node,
            bytes,
        );
    }

    match asn1.inner_asn1() {
        Asn1Type::Sequence(sequence) => {
            let set_cur_node = set_cur_node.clone();
            sequence
                .fields()
                .iter()
                .for_each(move |asn1| build_hex_bytes(asn1, cur_node, set_cur_node.clone(), bytes, select_all));
        }
        Asn1Type::OctetString(octet) => match octet.inner() {
            Some(asn1) => build_hex_bytes(asn1, cur_node, set_cur_node.clone(), bytes, select_all),
            None => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        },
        Asn1Type::Utf8String(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::BitString(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::BmpString(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::Bool(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::Null(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::Integer(_) => default_bytes(asn1_node_id, cur_node, set_cur_node, asn1, bytes, select_all),
        Asn1Type::ExplicitTag(explicit) => {
            build_hex_bytes(explicit.inner(), cur_node, set_cur_node.clone(), bytes, select_all)
        }
        Asn1Type::ApplicationTag(application) => {
            build_hex_bytes(application.inner(), cur_node, set_cur_node.clone(), bytes, select_all)
        }
    }
}
