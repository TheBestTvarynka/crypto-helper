use asn1_parser::OwnedSequence;
use yew::{Html, html, function_component, Properties};

use crate::asn1::scheme::build_asn1_schema;

#[derive(PartialEq, Properties, Clone)]
pub struct SequenceNodeProps {
    pub node: OwnedSequence,
}

#[function_component(SequenceNode)]
pub fn sequence(props: &SequenceNodeProps) -> Html {
    let fields = props.node.fields();

    let fields_components = fields.iter().map(|f| build_asn1_schema(f)).collect::<Vec<_>>();

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