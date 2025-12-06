use asn1_parser::{Asn1, Mutable};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::scheme::build_asn1_schema;

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1ViewerProps {
    pub structure: Mutable<Asn1>,
    pub re_encode: Callback<()>,

    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(Asn1Viewer)]
pub fn asn1_viewer(props: &Asn1ViewerProps) -> Html {
    let add_node = Callback::from(|_asn1_type| {
        // TODO: many asn1 trees.
    });

    html! {
        <div>
            {build_asn1_schema(&props.structure.get(), &props.cur_node, &props.set_cur_node, props.re_encode.clone(), add_node)}
        </div>
    }
}
