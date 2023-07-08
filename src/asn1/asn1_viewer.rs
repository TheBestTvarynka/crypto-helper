use asn1_parser::OwnedAsn1;
use yew::{Html, function_component, Properties, html};

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1ViewerProps {
    pub data: Vec<u8>,
    pub structure: OwnedAsn1,
}

#[function_component(Asn1Viewer)]
pub fn asn1_viewer(props: &Asn1ViewerProps) -> Html {
    html! {
        <div>
            {format!("{:?}", props.structure)}
        </div>
    }
}