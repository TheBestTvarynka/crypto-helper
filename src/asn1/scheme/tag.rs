use asn1_parser::{OwnedApplicationTag, OwnedExplicitTag, OwnedImplicitTag, OwnedRawAsn1EntityData};
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
#[derive(PartialEq, Properties, Clone)]
pub struct ImplicitTagProps {
    pub node: OwnedImplicitTag,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: OwnedRawAsn1EntityData,
}

#[function_component(ImplicitTagNode)]
pub fn implicit_tag(props: &ImplicitTagProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();
    let octets = props.node.octets();

    match props.node.inner_asn1() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("[{}] Implicit", props.node.tag_number())}/>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node)}
                </div>
            </div>
        },
        None => html! {
            <div class="terminal-asn1-node">
                <NodeOptions node_bytes={props.meta.raw_bytes().to_vec()} {offset} {length_len} {data_len} name={format!("[{}]", props.node.tag_number())} />
                <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                <span class="asn-simple-value">{hex::encode(octets)}</span>
            </div>
        },
    }
}
