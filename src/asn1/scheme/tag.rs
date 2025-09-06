use std::str::from_utf8;

use asn1_parser::{ApplicationTag, ExplicitTag, ImplicitTag, Mutable, RawAsn1EntityData};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::build_asn1_schema;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct ExplicitTagProps {
    pub node: Mutable<ExplicitTag>,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(ExplicitTagNode)]
pub fn explicit_tag(props: &ExplicitTagProps) -> Html {
    let set_cur_node = &props.set_cur_node;
    let inner_components = props
        .node
        .get()
        .inner()
        .iter()
        .map(|f| build_asn1_schema(f, &props.cur_node, set_cur_node, props.re_encode.clone()))
        .collect::<Vec<_>>();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={format!("[{}]", props.node.get().tag_number())}/>
            </div>
            <div class="asn1-constructor-body">
                {inner_components}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ApplicationTagProps {
    pub node: Mutable<ApplicationTag>,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(ApplicationTagNode)]
pub fn application_tag(props: &ApplicationTagProps) -> Html {
    let set_cur_node = &props.set_cur_node;
    let inner_components = props
        .node
        .get()
        .inner()
        .iter()
        .map(|f| build_asn1_schema(f, &props.cur_node, set_cur_node, props.re_encode.clone()))
        .collect::<Vec<_>>();

    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div style="cursor: crosshair; width: 100%">
            <div class="asn1-constructor-header">
                <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={format!("Application {}", props.node.get().tag_number())}/>
            </div>
            <div class="asn1-constructor-body">
                {inner_components}
            </div>
        </div>
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct ImplicitTagProps {
    pub node: Mutable<ImplicitTag>,
    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(ImplicitTagNode)]
pub fn implicit_tag(props: &ImplicitTagProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();
    let node = props.node.get();
    let octets = node.octets();

    match node.inner_asn1() {
        Some(asn1) => html! {
            <div style="cursor: crosshair; width: 100%">
                <div class="asn1-constructor-header">
                    <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={format!("[{}] Implicit", node.tag_number())}/>
                </div>
                <div class="asn1-constructor-body">
                    {build_asn1_schema(asn1, &props.cur_node, &props.set_cur_node, props.re_encode.clone())}
                </div>
            </div>
        },
        None => html! {
            <div class="terminal-asn1-node">
                <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={format!("[{}]", node.tag_number())} />
                <span class="asn1-node-info-label">{format!("({} bytes)", octets.len())}</span>
                {if let Ok(s) = from_utf8(octets) { html! {
                    <span class="asn-simple-value">{s}</span>
                }} else { html!{
                    <span class="asn-simple-value">{hex::encode(octets)}</span>
                }}}
            </div>
        },
    }
}
