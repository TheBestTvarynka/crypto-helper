use asn1_parser::{Asn1Entity, Asn1Type, OwnedAsn1};
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
    let cur_node = props.cur_node.clone();

    let set_cur_node = props.set_cur_node.clone();
    let onmouseenter = Callback::from(move |_: MouseEvent| {
        set_cur_node.emit(Some(3));
    });

    let set_cur_node = props.set_cur_node.clone();
    let onmouseleave = Callback::from(move |_: MouseEvent| {
        set_cur_node.emit(None);
    });

    html! {
        <div class="asn1-hex-viewer">
            <span {onmouseenter} {onmouseleave}>{"third elem show yourself"}</span>
            <div class="asn1-hex-node">
                {{
                    let set_cur_node = props.set_cur_node.clone();
                    let mut bytes = Vec::with_capacity(props.structure.raw_entity_data().raw_data.len());
                    build_hex_bytes(&props.structure, &mut bytes);
                    bytes
                }}
            </div>
        </div>
    }
}

fn format_bytes(bytes: &[u8], formatted_bytes: &mut Vec<VNode>) {
    bytes.iter().for_each(|byte| {
        formatted_bytes.push(html! {
            <span>{format!("{:02x?}", byte)}</span>
        })
    });
}

fn build_hex_bytes(asn1: &OwnedAsn1, bytes: &mut Vec<VNode>) {
    let tag: u8 = asn1.asn1().tag().into();
    bytes.push(html! {
        <span>{format!("{:02x?}", tag)}</span>
    });

    let raw = asn1.raw_entity_data().raw_bytes();
    format_bytes(&raw[asn1.raw_entity_data().length.clone()], bytes);

    build_data_bytes(asn1, bytes);
}

fn build_data_bytes(asn1: &OwnedAsn1, bytes: &mut Vec<VNode>) {
    fn default_bytes(asn1: &OwnedAsn1, bytes: &mut Vec<VNode>) {
        format_bytes(
            &asn1.raw_entity_data().raw_data[asn1.raw_entity_data().data.clone()],
            bytes,
        );
    }

    fn complex_bytes(asn1: &OwnedAsn1, bytes: &mut Vec<VNode>) {
        build_hex_bytes(asn1, bytes)
    }

    match asn1.asn1() {
        Asn1Type::Sequence(_) => default_bytes(asn1, bytes),
        Asn1Type::OctetString(_) => default_bytes(asn1, bytes),
        Asn1Type::Utf8String(_) => default_bytes(asn1, bytes),
        Asn1Type::BitString(_) => default_bytes(asn1, bytes),
        Asn1Type::BmpString(_) => default_bytes(asn1, bytes),
        Asn1Type::Bool(_) => default_bytes(asn1, bytes),
        Asn1Type::Null(_) => default_bytes(asn1, bytes),
        Asn1Type::ExplicitTag(_) => complex_bytes(asn1, bytes),
        Asn1Type::ApplicationTag(_) => complex_bytes(asn1, bytes),
    }
}
