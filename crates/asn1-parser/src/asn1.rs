use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decode, Asn1Encode, Asn1Entity, Asn1Result, Error, OctetString, Tag, Utf8String};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    // Sequence(Sequence<'data>),
    OctetString(OctetString<'data>),
    Utf8String(Utf8String<'data>),
}

impl Asn1Entity for Asn1Type<'_> {
    fn tag(&self) -> &crate::Tag {
        match self {
            // Asn1Type::Sequence(_) => todo!(),
            Asn1Type::OctetString(octet) => octet.tag(),
            Asn1Type::Utf8String(utf8) => utf8.tag(),
        }
    }
}

impl<'data> Asn1Decode<'data> for Asn1Type<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        OctetString::compare_tags(tag) || Utf8String::compare_tags(tag)
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag = Tag(reader.peek_byte()?);

        if OctetString::compare_tags(&tag) {
            Ok(Asn1Type::OctetString(OctetString::decode(reader)?))
        } else if Utf8String::compare_tags(&tag) {
            Ok(Asn1Type::Utf8String(Utf8String::decode(reader)?))
        } else {
            Err(Error::from("Invalid data"))
        }
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag = Tag(reader.peek_byte()?);

        if OctetString::compare_tags(&tag) {
            OctetString::decode_asn1(reader)
        } else if Utf8String::compare_tags(&tag) {
            Utf8String::decode_asn1(reader)
        } else {
            Err(Error::from("Invalid data"))
        }
    }
}

impl Asn1Encode for Asn1Type<'_> {
    fn needed_buf_size(&self) -> usize {
        match self {
            Asn1Type::OctetString(octet) => octet.needed_buf_size(),
            Asn1Type::Utf8String(utf8) => utf8.needed_buf_size(),
        }
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        match self {
            Asn1Type::OctetString(octet) => octet.encode(writer),
            Asn1Type::Utf8String(utf8) => utf8.encode(writer),
        }
    }
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
