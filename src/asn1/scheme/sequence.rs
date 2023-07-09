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
            <span>{format!("Sequence ({} fields)", fields.len())}</span>
            <div>
                {fields_components}
            </div>
        </div>
    }
}