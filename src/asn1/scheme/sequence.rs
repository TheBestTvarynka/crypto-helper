use asn1_parser::{Asn1, Asn1Type, Mutable, RawAsn1EntityData, Sequence};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::node_options::NodeOptions;
use crate::asn1::scheme::{AddNodeButton, build_asn1_schema};
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
    let sequence_node = props.node.clone();
    let re_encode = props.re_encode.clone();
    let fields_components = vec![html! {
        <div style="position: relative;">
            <AddNodeButton add_node={Callback::from(move |asn1_type: Asn1Type| {
                sequence_node.get_mut().fields_mut_vec().insert(0, Asn1::from_asn1_type(asn1_type));
                re_encode.emit(());
            })} />
        </div>
    }];
    let fields_components = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let re_encode = props.re_encode.clone();
            let sequence_node = props.node.clone();
            let add_node = Callback::from(move |asn1_type: Asn1Type| {
                sequence_node
                    .get_mut()
                    .fields_mut_vec()
                    .insert(i + 1, Asn1::from_asn1_type(asn1_type));
                re_encode.emit(());
            });

            let re_encode = props.re_encode.clone();
            let set_node = props.node.clone();
            let remove_node = Callback::from(move |_: ()| {
                set_node.get_mut().fields_mut_vec().remove(i);
                re_encode.emit(());
            });

            build_asn1_schema(
                f,
                &props.cur_node,
                set_cur_node,
                props.re_encode.clone(),
                add_node,
                remove_node,
            )
        })
        .fold(fields_components, |mut fields_components, component| {
            fields_components.push(component);
            fields_components
        });

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
