mod constructors;
mod generic_types;
mod string;

use asn1_parser::{Asn1Type, OwnedAsn1Type};
pub use constructors::*;
pub use generic_types::*;
use proptest::collection::vec;
use proptest::prelude::any;
use proptest::prop_oneof;
use proptest::strategy::Strategy;
pub use string::*;

use crate::constructors::recursive_empty_asn1_type;

pub fn bytes(size: usize) -> impl Strategy<Value = Vec<u8>> {
    vec(any::<u8>(), 0..size).no_shrink()
}

pub fn string(len: usize) -> impl Strategy<Value = String> {
    vec(any::<char>(), len)
        .prop_map(|v| v.iter().collect::<String>())
        .no_shrink()
}

#[allow(clippy::arc_with_non_send_sync)]
pub fn any_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    prop_oneof![
        any_octet_string().prop_map(Asn1Type::OctetString),
        any_utf8_string().prop_map(Asn1Type::Utf8String),
        any_bit_string().prop_map(Asn1Type::BitString),
        any_bmp_string().prop_map(Asn1Type::BmpString),
        any_bool().prop_map(Asn1Type::Bool),
        any_null().prop_map(Asn1Type::Null),
        recursive_empty_asn1_type(),
    ]
    .no_shrink()
}
