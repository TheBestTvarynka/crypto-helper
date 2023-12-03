mod primitive;
mod sequence;
mod strings;
mod tag;

use asn1_parser::{Asn1, Asn1Entity, Asn1Type};
use yew::virtual_dom::VNode;
use yew::{classes, function_component, html, Children, Classes, Html, Properties};

use self::primitive::{BoolNode, NullNode};
use self::sequence::SequenceNode;
use self::strings::{BitStringNode, BmpStringNode, OctetStringNode, Utf8StringNode};
use self::tag::{ApplicationTagNode, ExplicitTagNode};

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1NodeProps {
    pub id: u64,
    pub cur_id: Option<u64>,

    pub children: Children,
}

#[function_component(Asn1Node)]
pub fn asn1_node(props: &Asn1NodeProps) -> Html {
    fn compare_ids(id: u64, cur_id: &Option<u64>) -> Classes {
        match cur_id {
            Some(cur_id) if *cur_id == id => {
                classes!("hover_node")
            }
            _ => classes!(""),
        }
    }

    html! {
        <div class={compare_ids(props.id, &props.cur_id)}>
            {props.children.clone()}
        </div>
    }
}

pub fn build_asn1_schema(asn1: &Asn1<'_>, cur_node: &Option<u64>) -> VNode {
    match asn1.asn1() {
        Asn1Type::OctetString(octet) => html! {
            <Asn1Node id={octet.id()} cur_id={cur_node.clone()}>
                <OctetStringNode node={octet.to_owned()} />
            </Asn1Node>
        },
        Asn1Type::Utf8String(utf8) => html! {
            <Asn1Node id={utf8.id()} cur_id={cur_node.clone()}>
                <Utf8StringNode node={utf8.to_owned()} />
            </Asn1Node>
        },
        Asn1Type::Sequence(sequence) => html! {
            <Asn1Node id={sequence.id()} cur_id={cur_node.clone()}>
                <SequenceNode node={sequence.to_owned()} cur_node={cur_node.clone()} />
            </Asn1Node>
        },
        Asn1Type::BitString(bit) => html! {
            <Asn1Node id={bit.id()} cur_id={cur_node.clone()}>
                <BitStringNode node={bit.to_owned()} />
            </Asn1Node>
        },
        Asn1Type::Bool(boolean) => html! {
            <Asn1Node id={boolean.id()} cur_id={cur_node.clone()}>
                <BoolNode node={boolean.to_owned()} />
            </Asn1Node>
        },
        Asn1Type::BmpString(bmp) => html! {
            <Asn1Node id={bmp.id()} cur_id={cur_node.clone()}>
                <BmpStringNode node={bmp.to_owned()} />
            </Asn1Node>
        },
        Asn1Type::Null(null) => html! {
            <Asn1Node id={null.id()} cur_id={cur_node.clone()}>
                <NullNode />
            </Asn1Node>
        },
        Asn1Type::ExplicitTag(explicit) => html! {
            <Asn1Node id={explicit.id()} cur_id={cur_node.clone()}>
                <ExplicitTagNode node={explicit.to_owned()} cur_node={cur_node.clone()} />
            </Asn1Node>
        },
        Asn1Type::ApplicationTag(application) => html! {
            <Asn1Node id={application.id()} cur_id={cur_node.clone()}>
                <ApplicationTagNode node={application.to_owned()} cur_node={cur_node.clone()} />
            </Asn1Node>
        },
    }
}
