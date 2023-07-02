use alloc::vec::Vec;

use crate::Asn1Type;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sequence<'data> {
    fields: Vec<Asn1Type<'data>>,
}
