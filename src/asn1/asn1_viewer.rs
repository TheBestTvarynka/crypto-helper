use asn1_parser::OwnedAsn1;
use yew::{function_component, html, Html, Properties};

use crate::asn1::scheme::build_asn1_schema;

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1ViewerProps {
    pub data: Vec<u8>,
    pub structure: OwnedAsn1,
}

#[function_component(Asn1Viewer)]
pub fn asn1_viewer(props: &Asn1ViewerProps) -> Html {
    html! {
        <div>
            {build_asn1_schema(&props.structure)}
        </div>
    }
}
