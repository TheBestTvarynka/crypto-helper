use crate::Asn1Type;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sequence<'data> {
    fields: Vec<Asn1Type<'data>>,
}
