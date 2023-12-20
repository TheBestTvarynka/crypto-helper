use asn1_parser::{Bool, Null, OwnedInteger};
use proptest::prelude::any;
use proptest::strategy::{Just, Strategy};

use crate::bytes;

pub fn any_bool() -> impl Strategy<Value = Bool> {
    any::<bool>().prop_map(|flag| flag.into())
}

pub fn any_null() -> impl Strategy<Value = Null> {
    Just(Null)
}

pub fn any_integer() -> impl Strategy<Value = OwnedInteger> {
    bytes(1024).prop_map(|bytes| bytes.into())
}
