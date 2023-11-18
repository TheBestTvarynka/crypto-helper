mod primitive;
mod sequence;
mod strings;

use asn1_parser::{Asn1, Asn1Type};
use yew::html;
use yew::virtual_dom::VNode;

use self::sequence::SequenceNode;
use crate::asn1::scheme::primitive::BoolNode;
use crate::asn1::scheme::strings::{BitStringNode, OctetStringNode, Utf8StringNode};

// #[derive(PartialEq, Properties, Clone)]
// pub struct Asn1SchemeProps {
//     pub schema: OwnedAsn1,
// }

pub fn build_asn1_schema(asn1: &Asn1<'_>) -> VNode {
    match asn1.asn1() {
        Asn1Type::OctetString(octet) => html! {
            <OctetStringNode node={octet.to_owned()} />
        },
        Asn1Type::Utf8String(utf8) => html! {
            <Utf8StringNode node={utf8.to_owned()} />
        },
        Asn1Type::Sequence(sequence) => html! {
            <SequenceNode node={sequence.to_owned()} />
        },
        Asn1Type::BitString(bit) => html! {
            <BitStringNode node={bit.to_owned()} />
        },
        Asn1Type::Bool(boolean) => html! {
            <BoolNode node={boolean.to_owned()} />
        },
        // Asn1Type::ExplicitTag(e) => e.tag(),
        a => {
            log::error!("{:?}", a);
            // unimplemented!("{:?}", a)
            html! {
                <span>{format!("unimlemented: {:?}", a)}</span>
            }
        }
    }
}

// #[function_component(Asn1Scheme)]
// pub fn asn1_scheme(props: &Asn1SchemeProps) -> Html {
//     let node = ;

//     html! {
//         <div>
//         </div>
//     }
// }
