use asn1_parser::OwnedSequence;
use yew::{function_component, html, Callback, Html, Properties};

use crate::asn1::scheme::build_asn1_schema;
use crate::asn1::HighlightAction;

#[derive(PartialEq, Properties, Clone)]
pub struct SequenceNodeProps {
    pub node: OwnedSequence,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(SequenceNode)]
pub fn sequence(props: &SequenceNodeProps) -> Html {
    let fields = props.node.fields();

    let set_cur_node = &props.set_cur_node;
    let fields_components = fields
        .iter()
        .map(|f| build_asn1_schema(f, &props.cur_node, set_cur_node))
        .collect::<Vec<_>>();

    html! {
        <div>
            <div class="asn1-constructor-header">
                <span>{"Sequence"}</span>
                <span class="asn1-node-info-label">{format!("({} fields)", fields.len())}</span>
            </div>
            <div class="asn1-constructor-body">
                {fields_components}
            </div>
        </div>
    }
}
