use asn1_parser::OwnedAsn1;
use web_sys::MouseEvent;
use yew::{function_component, html, use_context, Callback, Html, Properties};

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
        log::debug!("Mouse enter event! {:?}", cur_node);
        set_cur_node.emit(Some(3));
    });

    let set_cur_node = props.set_cur_node.clone();
    let onmouseleave = Callback::from(move |_: MouseEvent| {
        log::debug!("Mouse leave event! {:?}", cur_node);
        set_cur_node.emit(None);
    });

    html! {
        <div class="asn1-hex-viewer">
            <span {onmouseenter} {onmouseleave}>{"third elem show yourself"}</span>
        </div>
    }
}
