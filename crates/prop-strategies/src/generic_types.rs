use asn1_parser::Bool;
use proptest::prelude::any;
use proptest::strategy::Strategy;

pub fn any_bool() -> impl Strategy<Value = Bool> {
    any::<bool>().prop_map(Bool::from)
}
