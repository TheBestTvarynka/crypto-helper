use asn1_parser::{Asn1Type, OwnedApplicationTag, OwnedAsn1, OwnedAsn1Type, OwnedExplicitTag, OwnedSequence};
use proptest::collection::vec;
use proptest::prop_oneof;
use proptest::strategy::{Just, Strategy};

use crate::{any_bit_string, any_bmp_string, any_bool, any_integer, any_null, any_octet_string, any_utf8_string};

fn any_leaf_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    prop_oneof![
        any_octet_string().prop_map(Asn1Type::OctetString),
        any_utf8_string().prop_map(Asn1Type::Utf8String),
        any_bit_string().prop_map(Asn1Type::BitString),
        any_bmp_string().prop_map(Asn1Type::BmpString),
        any_bool().prop_map(Asn1Type::Bool),
        any_null().prop_map(Asn1Type::Null),
        any_integer().prop_map(Asn1Type::Integer),
    ]
    .no_shrink()
}

pub fn recursive_empty_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    any_leaf_asn1_type().prop_recursive(16, 64, 32, |inner| {
        let explicit_tag_inner = inner.clone();
        let application_tag_inner = inner.clone();
        prop_oneof![
            vec(inner, 1..16).prop_map(|fields| {
                Asn1Type::Sequence(OwnedSequence::new(
                    fields
                        .into_iter()
                        .map(|asn1_type| OwnedAsn1::new(0, Default::default(), asn1_type))
                        .collect::<Vec<_>>(),
                ))
            }),
            (0_u8..31)
                .prop_flat_map(move |tag| (Just(tag), explicit_tag_inner.clone()))
                .prop_map(|(tag, inner)| Asn1Type::ExplicitTag(OwnedExplicitTag::new(
                    tag,
                    Box::new(OwnedAsn1::new(0, Default::default(), inner))
                ))),
            (0_u8..31)
                .prop_flat_map(move |tag| (Just(tag), application_tag_inner.clone()))
                .prop_map(|(tag, inner)| Asn1Type::ApplicationTag(OwnedApplicationTag::new(
                    tag,
                    Box::new(OwnedAsn1::new(0, Default::default(), inner))
                ))),
        ]
    })
}
