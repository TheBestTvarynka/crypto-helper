use yew::{classes, function_component, html, Html, Properties};

use crate::common::hex_format_byte;

#[derive(PartialEq, Eq, Properties)]
pub struct BytesViewerProps {
    pub bytes: Vec<u8>,
}

fn byte_color_class(byte: u8) -> &'static str {
    if byte == 0 {
        "byte-null"
    } else if byte.is_ascii_graphic() {
        "byte-printable"
    } else if byte.is_ascii_whitespace() {
        "byte-whitespace"
    } else if byte.is_ascii() {
        "byte-ascii"
    } else {
        "byte-other"
    }
}

fn byte_component(byte: u8) -> Html {
    html! {
        <span class={classes!(byte_color_class(byte))}>{hex_format_byte(byte)}</span>
    }
}

#[function_component(BytesViewer)]
pub fn bytes_viewer(props: &BytesViewerProps) -> Html {
    html! {
        <div class="bytes-container">{
            props.bytes.iter().map(|byte| byte_component(*byte)).collect::<Vec<_>>()
        }</div>
    }
}
