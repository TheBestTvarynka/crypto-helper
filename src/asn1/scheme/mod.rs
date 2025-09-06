mod oid;
mod primitive;
mod sequence;
mod set;
mod strings;
mod tag;
mod time;

use asn1_parser::{Asn1, Asn1Entity, Asn1Type};
use web_sys::MouseEvent;
use yew::virtual_dom::VNode;
use yew::{Callback, Children, Classes, Html, Properties, classes, function_component, html};

use self::oid::ObjectIdentifierNode;
use self::primitive::{BoolNode, EnumeratedNode, IntegerNode, NullNode};
use self::sequence::SequenceNode;
use self::strings::{
    BitStringNode, BmpStringNode, GeneralStringNode, IA5StringNode, NumericStringNode, OctetStringNode,
    PrintableStringNode, Utf8StringNode, VisibleStringNode,
};
use self::tag::{ApplicationTagNode, ExplicitTagNode, ImplicitTagNode};
use self::time::{GeneralizedTimeNode, UtcTimeNode};
use crate::asn1::HighlightAction;
use crate::asn1::scheme::set::SetNode;

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1NodeProps {
    pub id: u64,
    pub cur_id: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,

    pub children: Children,
}

#[function_component(Asn1Node)]
pub fn asn1_node(props: &Asn1NodeProps) -> Html {
    fn get_node_class(id: u64, cur_id: &Option<u64>) -> Classes {
        match cur_id {
            Some(cur_id) if *cur_id == id => {
                classes!("hover_node", "asn1-node-container")
            }
            _ => classes!("asn1-node-container"),
        }
    }

    let asn1_node_id = props.id;
    let set_cur_node_enter = props.set_cur_node.clone();
    let onmouseenter = Callback::from(move |_: MouseEvent| {
        set_cur_node_enter.emit(HighlightAction::Show(asn1_node_id));
    });
    let set_cur_node = props.set_cur_node.clone();
    let onmouseleave = Callback::from(move |_: MouseEvent| {
        set_cur_node.emit(HighlightAction::Hide(asn1_node_id));
    });

    html! {
        <div class={get_node_class(props.id, &props.cur_id)} {onmouseenter} {onmouseleave}>
            {props.children.clone()}
        </div>
    }
}

pub fn build_asn1_schema(
    asn1: &Asn1,
    cur_id: &Option<u64>,
    set_cur_node: &Callback<HighlightAction>,
    re_encode: Callback<()>,
) -> VNode {
    match asn1.inner_asn1() {
        Asn1Type::OctetString(octet) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <OctetStringNode node={octet.clone()} meta={asn1.meta().clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Utf8String(utf8) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <Utf8StringNode node={utf8.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::IA5String(ia5) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <IA5StringNode node={ia5.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::PrintableString(printable) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <PrintableStringNode node={printable.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::GeneralString(general) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <GeneralStringNode node={general.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::NumericString(numeric) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <NumericStringNode node={numeric.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::VisibleString(visible) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <VisibleStringNode node={visible.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Sequence(sequence) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <SequenceNode node={sequence.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Set(set) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <SetNode node={set.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::BitString(bit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <BitStringNode node={bit.clone()} meta={asn1.meta().clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Bool(boolean) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <BoolNode node={boolean.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::BmpString(bmp) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <BmpStringNode node={bmp.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::Null(_) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <NullNode meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::Integer(integer) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <IntegerNode node={integer.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::Enumerated(enumerated) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <EnumeratedNode node={enumerated.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::ObjectIdentifier(object_identifier) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <ObjectIdentifierNode node={object_identifier.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::ExplicitTag(explicit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <ExplicitTagNode node={explicit.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ImplicitTag(implicit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <ImplicitTagNode node={implicit.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ApplicationTag(application) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <ApplicationTagNode node={application.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::UtcTime(utc_time) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <UtcTimeNode node={utc_time.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::GeneralizedTime(generalized_time) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()}>
                <GeneralizedTimeNode node={generalized_time.clone()} meta={asn1.meta().clone()} />
            </Asn1Node>
        },
    }
}
