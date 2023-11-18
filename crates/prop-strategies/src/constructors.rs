use asn1_parser::{Asn1Type, OwnedAsn1, OwnedAsn1Type, OwnedSequence};
use proptest::collection::vec;
use proptest::prop_oneof;
use proptest::strategy::Strategy;

use crate::{any_bit_string, any_bool, any_octet_string, any_utf8_string};

fn any_leaf_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    prop_oneof![
        any_octet_string().prop_map(Asn1Type::OctetString),
        any_utf8_string().prop_map(Asn1Type::Utf8String),
        any_bit_string().prop_map(Asn1Type::BitString),
        any_bool().prop_map(Asn1Type::Bool),
    ]
    .no_shrink()
}

pub fn recursive_empty_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    any_leaf_asn1_type().prop_recursive(16, 64, 32, |inner| {
        vec(inner, 1..16).prop_map(|fields| {
            Asn1Type::Sequence(OwnedSequence::from(
                fields
                    .into_iter()
                    .map(|asn1_type| OwnedAsn1::new(Default::default(), Box::new(asn1_type)))
                    .collect::<Vec<_>>(),
            ))
        })
    })
}
