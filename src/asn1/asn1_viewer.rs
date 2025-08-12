use asn1_parser::OwnedAsn1;
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::scheme::build_asn1_schema;

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1ViewerProps {
    pub structure: OwnedAsn1,

    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(Asn1Viewer)]
pub fn asn1_viewer(props: &Asn1ViewerProps) -> Html {
    html! {
        <div>
            {build_asn1_schema(&props.structure, &props.cur_node, &props.set_cur_node)}
        </div>
    }
}
