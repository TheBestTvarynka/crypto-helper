use asn1_parser::{Bool, Enumerated, Integer, Mutable, RawAsn1EntityData};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::editor::{IntegerEditor, NullEditor};
use crate::asn1::node_options::NodeOptions;
use crate::common::{RcSlice, Switch};

#[derive(PartialEq, Properties, Clone)]
pub struct BoolNodeProps {
    pub node: Mutable<Bool>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(BoolNode)]
pub fn bool(props: &BoolNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    let node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: bool| {
        node.get_mut().set(value);
        re_encode.emit(());
    });

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions
                node_bytes={RcSlice::from(props.meta.raw_bytes())}
                {offset}
                {length_len}
                {data_len}
                name={String::from("Bool")}
                editor={Some(html! {
                    <Switch id="bool_editor" state={props.node.get().value()} {setter} />
                })}
            />
            {if props.node.get().value() {html! {
                <span class="asn-bool-true">{"true"}</span>
            }} else {html! {
                <span class="asn-bool-false">{"false"}</span>
            }}}
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct NullNodeProps {
    pub meta: RawAsn1EntityData,
}

#[function_component(NullNode)]
pub fn null(props: &NullNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions
                node_bytes={RcSlice::from(props.meta.raw_bytes())}
                {offset}
                {length_len}
                {data_len}
                name={String::from("Null")}
                editor={Some(html! { <NullEditor /> })}
            />
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct IntegerNodeProps {
    pub node: Mutable<Integer>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(IntegerNode)]
pub fn integer(props: &IntegerNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    let node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: Vec<u8>| {
        node.get_mut().set(value);
        re_encode.emit(());
    });

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions
                node_bytes={RcSlice::from(props.meta.raw_bytes())}
                {offset}
                {length_len}
                {data_len}
                name={String::from("Integer")}
                editor={Some(html! {
                    <IntegerEditor value={props.node.get().raw_data().to_vec()} {setter} />
                })}
            />
            <span class="asn-simple-value">{format!("{}", props.node.get().as_big_uint())}</span>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EnumeratedNodeProps {
    pub node: Mutable<Enumerated>,
    pub meta: RawAsn1EntityData,
    pub re_encode: Callback<()>,
}

#[function_component(EnumeratedNode)]
pub fn enumerated(props: &EnumeratedNodeProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    let node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let setter = Callback::from(move |value: Vec<u8>| {
        node.get_mut().set(value);
        re_encode.emit(());
    });

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions
                node_bytes={RcSlice::from(props.meta.raw_bytes())}
                {offset}
                {length_len}
                {data_len}
                name={String::from("Enumerated")}
                editor={Some(html! {
                    <IntegerEditor value={props.node.get().raw_data().to_vec()} {setter} />
                })}
            />
            <span class="asn-simple-value">{format!("{}", props.node.get().as_big_uint())}</span>
        </div>
    }
}
