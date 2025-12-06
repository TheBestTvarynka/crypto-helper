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
use yew::{Callback, Children, Classes, Html, Properties, classes, function_component, html, use_state};

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
use crate::asn1::editor::{NodeValueEditor, RemoveNodeConfirmation};
use crate::asn1::scheme::set::SetNode;

#[derive(PartialEq, Properties, Clone)]
pub struct AddNodeButtonProps {
    pub add_node: Callback<Asn1Type>,
}

#[function_component(AddNodeButton)]
pub fn add_node_button(props: &AddNodeButtonProps) -> Html {
    let show_panel = use_state(|| false);

    let onclick = {
        let show_panel = show_panel.clone();
        Callback::from(move |_: MouseEvent| {
            show_panel.set(!*show_panel);
        })
    };
    let hide_panel = Callback::from({
        let show_panel = show_panel.clone();
        move |_| {
            show_panel.set(false);
        }
    });

    let show_panel_setter = show_panel.setter();
    let add_node = props.add_node.clone();
    let add_node = Callback::from(move |asn1: Asn1Type| {
        show_panel_setter.set(false);
        add_node.emit(asn1);
    });

    html! {
        <div class="asn1-node-edit-options">
            <button class="asn1-button-add-node" {onclick}>
                <img src="/public/img/icons/add.png" />
                <div style="position: relative;">
                    <span
                        class="asn1-button-add-separator"
                        style="position: absolute; bottom: 0; right: 0; transform: translate(100%, 50%);"
                    />
                </div>
            </button>
            {if *show_panel {html! {
                <div style="position: relative">
                    <div class="asn1-node-options">
                        <NodeValueEditor {add_node} cancel={hide_panel} />
                    </div>
                </div>
            }} else {html! {}}}
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct RemoveNodeButtonProps {
    pub remove_node: Callback<()>,
}

#[function_component(RemoveNodeButton)]
pub fn remove_node_button(props: &RemoveNodeButtonProps) -> Html {
    let show_panel = use_state(|| false);

    let onclick = {
        let show_panel = show_panel.clone();
        Callback::from(move |_: MouseEvent| {
            show_panel.set(true);
        })
    };

    let show_panel_setter = show_panel.setter();
    let remove_node = props.remove_node.clone();
    let remove_node = Callback::from(move |_: ()| {
        show_panel_setter.set(false);
        remove_node.emit(());
    });

    let show_panel_setter = show_panel.setter();
    let onmouseleave = Callback::from(move |_| {
        show_panel_setter.set(false);
    });

    html! {
        <div class="asn1-remove-node-button-container">
            <button class="asn1-button-add-node" {onclick}>
                <img src="/public/img/icons/remove.png" />
            </button>
            {if *show_panel {html! {
                <div style="position: relative" {onmouseleave}>
                    <div class="asn1-node-options">
                        <RemoveNodeConfirmation {remove_node} />
                    </div>
                </div>
            }} else {html! {}}}
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1NodeProps {
    pub id: u64,
    pub cur_id: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
    pub add_node: Callback<Asn1Type>,
    pub remove_node: Callback<()>,

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
        <div class={get_node_class(props.id, &props.cur_id)} {onmouseenter} {onmouseleave} style="position: relative;">
            {props.children.clone()}
            <AddNodeButton add_node={props.add_node.clone()} />
            <RemoveNodeButton remove_node={props.remove_node.clone()} />
        </div>
    }
}

pub fn build_asn1_schema(
    asn1: &Asn1,
    cur_id: &Option<u64>,
    set_cur_node: &Callback<HighlightAction>,
    re_encode: Callback<()>,
    add_node: Callback<Asn1Type>,
    remove_node: Callback<()>,
) -> VNode {
    match asn1.inner_asn1() {
        Asn1Type::OctetString(octet) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <OctetStringNode node={octet.clone()} meta={asn1.meta().clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Utf8String(utf8) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <Utf8StringNode node={utf8.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::IA5String(ia5) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <IA5StringNode node={ia5.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::PrintableString(printable) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <PrintableStringNode node={printable.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::GeneralString(general) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <GeneralStringNode node={general.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::NumericString(numeric) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <NumericStringNode node={numeric.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::VisibleString(visible) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <VisibleStringNode node={visible.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Sequence(sequence) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <SequenceNode node={sequence.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Set(set) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <SetNode node={set.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::BitString(bit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <BitStringNode node={bit.clone()} meta={asn1.meta().clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Bool(boolean) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <BoolNode node={boolean.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::BmpString(bmp) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <BmpStringNode node={bmp.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Null(_) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <NullNode meta={asn1.meta().clone()} />
            </Asn1Node>
        },
        Asn1Type::Integer(integer) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <IntegerNode node={integer.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::Enumerated(enumerated) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <EnumeratedNode node={enumerated.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ObjectIdentifier(object_identifier) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <ObjectIdentifierNode node={object_identifier.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ExplicitTag(explicit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <ExplicitTagNode node={explicit.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ImplicitTag(implicit) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <ImplicitTagNode node={implicit.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::ApplicationTag(application) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <ApplicationTagNode node={application.clone()} cur_node={cur_id} set_cur_node={set_cur_node.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::UtcTime(utc_time) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <UtcTimeNode node={utc_time.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
        Asn1Type::GeneralizedTime(generalized_time) => html! {
            <Asn1Node id={asn1.id()} {cur_id} set_cur_node={set_cur_node.clone()} {add_node} {remove_node}>
                <GeneralizedTimeNode node={generalized_time.clone()} meta={asn1.meta().clone()} re_encode={re_encode.clone()} />
            </Asn1Node>
        },
    }
}
