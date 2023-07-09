mod sequence;
mod strings;

use asn1_parser::{Asn1Type, Asn1};
use yew::{function_component, html, Html, Properties, virtual_dom::VNode};

use crate::asn1::scheme::strings::{OctetStringNode, Utf8StringNode};

use self::sequence::SequenceNode;

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
        // Asn1Type::BitString(bit) => bit.tag(),
        // Asn1Type::Bool(boolean) => boolean.tag(),
        // Asn1Type::ExplicitTag(e) => e.tag(),
        _ => unimplemented!(""),
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
