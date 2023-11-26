use yew::{function_component, html, Html, Properties};

#[function_component(HexViewer)]
pub fn hex_viewer() -> Html {
    html! {
        <div class="asn1-hex-viewer">
            <span>{"HexView"}</span>
        </div>
    }
}
