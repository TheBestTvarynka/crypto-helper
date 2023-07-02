use alloc::vec::Vec;

use crate::Asn1Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence<'data> {
    fields: Vec<Asn1Type<'data>>,
}

impl Default for Sequence<'_> {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}
