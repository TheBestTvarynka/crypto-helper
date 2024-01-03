use asn1_parser::{Bool, Null, ObjectIdentifier, OwnedInteger};
use proptest::collection::vec;
use proptest::prelude::any;
use proptest::prop_compose;
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

prop_compose! {
    pub fn any_object_identifier()
        (
            first_node in 0..3,
            second_node in 0..40,
            all_nodes in 2_usize..11,
        )
        (
            first_node in Just(first_node),
            second_node in Just(second_node),
            nodes in vec(0..u32::MAX - 1, all_nodes)
        ) -> ObjectIdentifier {
        let mut formatted_oid = format!("{}.{}", dbg!(first_node), dbg!(second_node));
        for node in nodes {
            formatted_oid.push_str(&format!(".{}", node));
        }
        ObjectIdentifier::from(oid::ObjectIdentifier::try_from(formatted_oid).expect("Valid object identifier."))
    }
}
