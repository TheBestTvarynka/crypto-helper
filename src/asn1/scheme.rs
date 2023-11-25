mod primitive;
mod sequence;
mod strings;
mod tag;

use asn1_parser::{Asn1, Asn1Type};
use yew::html;
use yew::virtual_dom::VNode;

use self::primitive::{BoolNode, NullNode};
use self::sequence::SequenceNode;
use self::strings::{BitStringNode, BmpStringNode, OctetStringNode, Utf8StringNode};
use self::tag::{ApplicationTagNode, ExplicitTagNode};

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
        Asn1Type::BmpString(bmp) => html! {
            <BmpStringNode node={bmp.to_owned()} />
        },
        Asn1Type::Null(_) => html! {
            <NullNode />
        },
        Asn1Type::ExplicitTag(explicit) => html! {
            <ExplicitTagNode node={explicit.to_owned()} />
        },
        Asn1Type::ApplicationTag(application) => html! {
            <ApplicationTagNode node={application.to_owned()} />
        },
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
