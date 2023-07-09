use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, BitString, Bool, Error, ExplicitTag, OctetString, Sequence, Tag,
    Utf8String,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    Sequence(Sequence<'data>),
    OctetString(OctetString<'data>),
    Utf8String(Utf8String<'data>),
    BitString(BitString<'data>),

    Bool(Bool),

    ExplicitTag(ExplicitTag<'data>),
}

pub type OwnedAsn1Type = Asn1Type<'static>;

impl Asn1Type<'_> {
    pub fn to_owned(&self) -> OwnedAsn1Type {
        match self {
            Asn1Type::Sequence(s) => Asn1Type::Sequence(s.to_owned()),
            Asn1Type::OctetString(o) => Asn1Type::OctetString(o.to_owned()),
            Asn1Type::Utf8String(u) => Asn1Type::Utf8String(u.to_owned()),
            // Asn1Type::BitString(b) => Asn1Type::BitString(b.to_owned()),
            // Asn1Type::Bool(b) => Asn1Type::Bool(b),
            // Asn1Type::ExplicitTag(_) => todo!(),
            _ => unimplemented!(),
        }
    }
}

impl Asn1Entity for Asn1Type<'_> {
    fn tag(&self) -> Tag {
        match self {
            Asn1Type::OctetString(octet) => octet.tag(),
            Asn1Type::Utf8String(utf8) => utf8.tag(),
            Asn1Type::Sequence(sequence) => sequence.tag(),
            Asn1Type::BitString(bit) => bit.tag(),
            Asn1Type::Bool(boolean) => boolean.tag(),
            Asn1Type::ExplicitTag(e) => e.tag(),
        }
    }
}

impl<'data> Asn1Decoder<'data> for Asn1Type<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        OctetString::compare_tags(tag) || Utf8String::compare_tags(tag)
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag = Tag(reader.peek_byte()?);

        if OctetString::compare_tags(&tag) {
            Ok(Asn1Type::OctetString(OctetString::decode(reader)?))
        } else if Utf8String::compare_tags(&tag) {
            Ok(Asn1Type::Utf8String(Utf8String::decode(reader)?))
        } else if Sequence::compare_tags(&tag) {
            Ok(Asn1Type::Sequence(Sequence::decode(reader)?))
        } else if BitString::compare_tags(&tag) {
            Ok(Asn1Type::BitString(BitString::decode(reader)?))
        } else if Bool::compare_tags(&tag) {
            Ok(Asn1Type::Bool(Bool::decode(reader)?))
        } else if ExplicitTag::compare_tags(&tag) {
            Ok(Asn1Type::ExplicitTag(ExplicitTag::decode(reader)?))
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
        } else if Sequence::compare_tags(&tag) {
            Sequence::decode_asn1(reader)
        } else if BitString::compare_tags(&tag) {
            BitString::decode_asn1(reader)
        } else if Bool::compare_tags(&tag) {
            Bool::decode_asn1(reader)
        } else if ExplicitTag::compare_tags(&tag) {
            ExplicitTag::decode_asn1(reader)
        } else {
            Err(Error::from("Invalid data"))
        }
    }
}

impl Asn1Encoder for Asn1Type<'_> {
    fn needed_buf_size(&self) -> usize {
        match self {
            Asn1Type::OctetString(octet) => octet.needed_buf_size(),
            Asn1Type::Utf8String(utf8) => utf8.needed_buf_size(),
            Asn1Type::Sequence(sequence) => sequence.needed_buf_size(),
            Asn1Type::BitString(bit) => bit.needed_buf_size(),
            Asn1Type::Bool(boolean) => boolean.needed_buf_size(),
            Asn1Type::ExplicitTag(e) => e.needed_buf_size(),
        }
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        match self {
            Asn1Type::OctetString(octet) => octet.encode(writer),
            Asn1Type::Utf8String(utf8) => utf8.encode(writer),
            Asn1Type::Sequence(sequence) => sequence.encode(writer),
            Asn1Type::BitString(bit) => bit.encode(writer),
            Asn1Type::Bool(boolean) => boolean.encode(writer),
            Asn1Type::ExplicitTag(e) => e.encode(writer),
        }
    }
}

/// Information about raw data of the asn1 entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawAsn1EntityData<'data> {
    /// Raw input bytes
    pub raw_data: Cow<'data, [u8]>,

    /// Position of the tag in the input data
    pub tag: usize,

    /// Range that corresponds to the encoded length bytes
    pub length: Range<usize>,

    /// Range that corresponds to the inner raw data
    pub data: Range<usize>,
}

pub type OwnedRawAsn1EntityData = RawAsn1EntityData<'static>;

impl RawAsn1EntityData<'_> {
    pub fn tag_position(&self) -> usize {
        self.tag
    }

    pub fn length_range(&self) -> Range<usize> {
        self.length.clone()
    }

    pub fn data_range(&self) -> Range<usize> {
        self.data.clone()
    }

    pub fn raw_bytes(&self) -> &[u8] {
        self.raw_data.as_ref()
    }

    pub fn length_bytes(&self) -> &[u8] {
        &self.raw_data[self.length.clone()]
    }

    pub fn data_bytes(&self) -> &[u8] {
        &self.raw_data[self.data.clone()]
    }

    pub fn to_owned(&self) -> OwnedRawAsn1EntityData {
        RawAsn1EntityData {
            raw_data: self.raw_data.to_vec().into(),
            tag: self.tag,
            length: self.length.clone(),
            data: self.data.clone(),
        }
    }
}

/// [`Asn1`] structure represents generic `asn1` value.
/// It contains raw data and parsed values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asn1<'data> {
    /// Information about raw data of the asn1
    pub(crate) raw_data: RawAsn1EntityData<'data>,

    /// Parsed asn1 data
    pub(crate) asn1_type: Box<Asn1Type<'data>>,
}

pub type OwnedAsn1 = Asn1<'static>;

impl Asn1<'_> {
    pub fn raw_entity_data(&self) -> &RawAsn1EntityData<'_> {
        &self.raw_data
    }

    pub fn asn1(&self) -> &Asn1Type<'_> {
        &self.asn1_type
    }

    pub fn to_owned(&self) -> OwnedAsn1 {
        Asn1 {
            raw_data: self.raw_data.to_owned(),
            asn1_type: Box::new(self.asn1_type.to_owned()),
        }
    }
}

impl Default for Asn1<'_> {
    fn default() -> Self {
        // those values are just for testing purpose during development
        Asn1Type::decode_asn1_buff(&[
48, 90, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 12, 7, 113, 107, 97, 116, 105, 111, 110, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156, 221, 80, 86, 181, 110, 48, 44, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 12, 7, 113, 107, 97, 116, 105, 111, 110, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156, 221, 80, 86, 181, 110
        ])
        .unwrap()
    }
}
