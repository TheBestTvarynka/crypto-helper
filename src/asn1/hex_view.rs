use asn1_parser::{Asn1, Asn1Entity, Asn1Type, OwnedAsn1};
use web_sys::MouseEvent;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct HexViewerProps {
    pub raw_data: Vec<u8>,
    pub structure: OwnedAsn1,

    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<Option<u64>>,
}

#[function_component(HexViewer)]
pub fn hex_viewer(props: &HexViewerProps) -> Html {
    html! {
        <div class="asn1-hex-viewer">
            <div class="asn1-hex-node">
                {{
                    let set_cur_node = props.set_cur_node.clone();
                    let mut bytes = Vec::with_capacity(props.structure.raw_entity_data().raw_data.len());
                    build_hex_bytes(&props.raw_data, &props.structure, set_cur_node, &mut bytes);
                    bytes
                }}
            </div>
        </div>
    }
}

fn format_bytes(
    bytes: &[u8],
    asn1_node_id: u64,
    set_cur_node: Callback<Option<u64>>,
    formatted_bytes: &mut Vec<VNode>,
) {
    bytes.iter().for_each(|byte| {
        let set_cur_node_enter = set_cur_node.clone();
        let onmouseenter = Callback::from(move |_: MouseEvent| set_cur_node_enter.emit(Some(asn1_node_id)));
        let set_cur_node = set_cur_node.clone();
        let onmouseleave = Callback::from(move |_: MouseEvent| set_cur_node.emit(None));
        formatted_bytes.push(html! {
            <span {onmouseenter} {onmouseleave}>{format!("{:02x?}", byte)}</span>
        })
    });
}

fn build_hex_bytes(raw: &[u8], asn1: &Asn1<'_>, set_cur_node: Callback<Option<u64>>, bytes: &mut Vec<VNode>) {
    let asn1_node_id = asn1.asn1().id();

    let tag: u8 = asn1.asn1().tag().into();
    let tag_set_cur_node = set_cur_node.clone();
    let onmouseenter = Callback::from(move |_: MouseEvent| tag_set_cur_node.emit(Some(asn1_node_id)));
    let tag_set_cur_node = set_cur_node.clone();
    let onmouseleave = Callback::from(move |_: MouseEvent| tag_set_cur_node.emit(None));
    bytes.push(html! {
        <span {onmouseenter} {onmouseleave}>{format!("{:02x?}", tag)}</span>
    });

    format_bytes(
        &raw[asn1.raw_entity_data().length.clone()],
        asn1_node_id,
        set_cur_node.clone(),
        bytes,
    );

    build_data_bytes(raw, asn1, asn1_node_id, set_cur_node, bytes);
}

fn build_data_bytes(
    raw: &[u8],
    asn1: &Asn1<'_>,
    asn1_node_id: u64,
    set_cur_node: Callback<Option<u64>>,
    bytes: &mut Vec<VNode>,
) {
    fn default_bytes(
        raw: &[u8],
        asn1_node_id: u64,
        set_cur_node: Callback<Option<u64>>,
        asn1: &Asn1<'_>,
        bytes: &mut Vec<VNode>,
    ) {
        format_bytes(
            &raw[asn1.raw_entity_data().data.clone()],
            asn1_node_id,
            set_cur_node,
            bytes,
        );
    }

    match asn1.asn1() {
        Asn1Type::Sequence(sequence) => {
            let set_cur_node = set_cur_node.clone();
            sequence
                .fields()
                .iter()
                .for_each(move |asn1| build_hex_bytes(raw, asn1, set_cur_node.clone(), bytes));
        }
        Asn1Type::OctetString(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::Utf8String(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::BitString(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::BmpString(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::Bool(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::Null(_) => default_bytes(raw, asn1_node_id, set_cur_node, asn1, bytes),
        Asn1Type::ExplicitTag(explicit) => build_hex_bytes(raw, explicit.inner(), set_cur_node.clone(), bytes),
        Asn1Type::ApplicationTag(application) => build_hex_bytes(raw, application.inner(), set_cur_node.clone(), bytes),
    }
}
