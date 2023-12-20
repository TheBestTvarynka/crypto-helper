use asn1_parser::{OwnedApplicationTag, OwnedExplicitTag, OwnedRawAsn1EntityData};
use yew::{function_component, html, Callback, Html, Properties};

use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::asn1::HighlightAction;

#[derive(PartialEq, Properties, Clone)]
pub struct ExplicitTagProps {
    pub node: OwnedExplicitTag,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(ExplicitTagNode)]
pub fn explicit_tag(props: &ExplicitTagProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("[{}]", props.node.tag_number())}/>
            </div>
            <div class="asn1-constructor-body">
                {build_asn1_schema(props.node.inner(), &props.cur_node, &props.set_cur_node)}
            </div>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct ApplicationTagProps {
    pub node: OwnedApplicationTag,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(ApplicationTagNode)]
pub fn application_tag(props: &ApplicationTagProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("Application {}", props.node.tag_number())}/>
            </div>
            <div class="asn1-constructor-body">
                {build_asn1_schema(props.node.inner(), &props.cur_node, &props.set_cur_node)}
            </div>
        </div>
    }
}
