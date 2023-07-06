use asn1_parser::Bool;
use proptest::strategy::Strategy;
use proptest::prelude::any;

pub fn any_bool() -> impl Strategy<Value = Bool> {
    any::<bool>().prop_map(Bool::from)
}