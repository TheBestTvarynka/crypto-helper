use asn1_parser::{Bool, Null};
use proptest::prelude::any;
use proptest::strategy::{Just, Strategy};

pub fn any_bool() -> impl Strategy<Value = Bool> {
    any::<bool>().prop_map(Bool::from)
}

pub fn any_null() -> impl Strategy<Value = Null> {
    Just(Null)
}
