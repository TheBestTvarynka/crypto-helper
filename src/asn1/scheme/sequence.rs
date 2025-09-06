use asn1_parser::{Mutable, RawAsn1EntityData, Sequence};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct SequenceNodeProps {
    pub node: Mutable<Sequence>,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(SequenceNode)]
pub fn sequence(props: &SequenceNodeProps) -> Html {
    let fields = props.node.get();
    let fields = fields.fields();

    let set_cur_node = &props.set_cur_node;
    let fields_components = fields
        .iter()
        .map(|f| build_asn1_schema(f, &props.cur_node, set_cur_node, props.re_encode.clone()))
        .collect::<Vec<_>>();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("Sequence")}/>
                <span class="asn1-node-info-label">{format!("({} fields)", fields.len())}</span>
            </div>
            <div class="asn1-constructor-body">
                {fields_components}
            </div>
        </div>
    }
}
