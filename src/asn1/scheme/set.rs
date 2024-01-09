use asn1_parser::{OwnedRawAsn1EntityData, OwnedSet};
use yew::{function_component, html, Callback, Html, Properties};

use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::asn1::HighlightAction;

#[derive(PartialEq, Properties, Clone)]
pub struct SetNodeProps {
    pub node: OwnedSet,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(SetNode)]
pub fn set(props: &SetNodeProps) -> Html {
    let fields = props.node.fields();

    let set_cur_node = &props.set_cur_node;
    let fields_components = fields
        .iter()
        .map(|f| build_asn1_schema(f, &props.cur_node, set_cur_node))
        .collect::<Vec<_>>();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={String::from("Set")}/>
                <span class="asn1-node-info-label">{format!("({} fields)", fields.len())}</span>
            </div>
            <div class="asn1-constructor-body">
                {fields_components}
            </div>
        </div>
    }
}
