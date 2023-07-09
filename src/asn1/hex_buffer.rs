use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct HexBufferProps {
    pub data: Vec<u8>,
}

#[function_component(HexBuffer)]
pub fn hex_buffer(props: &HexBufferProps) -> Html {
    html! {
        <div>
            {"hex buffer"}
        </div>
    }
}
