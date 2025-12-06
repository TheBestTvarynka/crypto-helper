use asn1_parser::{Asn1, Asn1Type, Mutable};
use yew::{Callback, Html, Properties, function_component, html};

use crate::asn1::HighlightAction;
use crate::asn1::scheme::{AddNodeButton, build_asn1_schema};

#[derive(PartialEq, Properties, Clone)]
pub struct Asn1ViewerProps {
    pub structures: Mutable<Vec<Asn1>>,
    pub re_encode: Callback<()>,

    pub cur_node: Option<u64>,
    pub set_cur_node: Callback<HighlightAction>,
}

#[function_component(Asn1Viewer)]
pub fn asn1_viewer(props: &Asn1ViewerProps) -> Html {
    let set_cur_node = &props.set_cur_node;

    let structures = props.structures.clone();
    let re_encode = props.re_encode.clone();
    let trees = vec![html! {
        <div style="position: relative;">
            <AddNodeButton add_node={Callback::from(move |asn1_type: Asn1Type| {
                structures.get_mut().insert(0, Asn1::from_asn1_type(asn1_type));
                re_encode.emit(());
            })} />
        </div>
    }];

    let trees = props
        .structures
        .get()
        .iter()
        .enumerate()
        .map(|(i, structure)| {
            let re_encode = props.re_encode.clone();
            let structures = props.structures.clone();
            let add_node = Callback::from(move |asn1_type: Asn1Type| {
                structures.get_mut().insert(i + 1, Asn1::from_asn1_type(asn1_type));
                re_encode.emit(());
            });

            let re_encode = props.re_encode.clone();
            let structures = props.structures.clone();
            let remove_node = Callback::from(move |_: ()| {
                structures.get_mut().remove(i);
                re_encode.emit(());
            });

            build_asn1_schema(
                structure,
                &props.cur_node,
                set_cur_node,
                props.re_encode.clone(),
                add_node,
                remove_node,
            )
        })
        .fold(trees, |mut trees, component| {
            trees.push(component);
            trees
        });

    html! {
        <div>
            {trees}
        </div>
    }
}
