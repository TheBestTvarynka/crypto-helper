use core::ops::Range;

use crate::{OctetString, Sequence, Utf8String};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    Sequence(Sequence<'data>),
    OctetString(OctetString<'data>),
    Utf8String(Utf8String<'data>),
}

/// [`Asn1`] structure represents generic `asn1` value.
/// It contains raw data and parsed values.
#[derive(Debug)]
pub struct Asn1<'data> {
    /// Raw input bytes
    pub(crate) raw_data: &'data [u8],

    /// Position of the tag in the input data
    pub(crate) tag: usize,

    /// Range that corresponds to the encoded length bytes
    pub(crate) length: Range<usize>,

    /// Range that corresponds to the inner raw data
    pub(crate) data: Range<usize>,

    /// Parsed asn1 data
    pub(crate) asn1_type: Asn1Type<'data>,
}

impl Asn1<'_> {
    pub fn tag_position(&self) -> usize {
        self.tag
    }

    pub fn length_range(&self) -> Range<usize> {
        self.length.clone()
    }

    pub fn data_range(&self) -> Range<usize> {
        self.data.clone()
    }

    pub fn length_bytes(&self) -> &[u8] {
        &self.raw_data[self.length.clone()]
    }

    pub fn data_bytes(&self) -> &[u8] {
        &self.raw_data[self.data.clone()]
    }

    pub fn asn1(&self) -> &Asn1Type<'_> {
        &self.asn1_type
    }
}

impl<'data> Asn1<'data> {
    pub fn parse(_input: &'data [u8]) -> Self {
        todo!()
    }
}
