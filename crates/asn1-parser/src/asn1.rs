use alloc::borrow::Cow;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    ApplicationTag, Asn1Encoder, Asn1Result, Asn1ValueDecoder, BitString, BmpString, Bool, Error, ExplicitTag, Integer,
    Null, OctetString, Sequence, Tag, Tlv, Utf8String,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    Sequence(Sequence<'data>),
    OctetString(OctetString<'data>),
    Utf8String(Utf8String<'data>),
    BitString(BitString<'data>),
    BmpString(BmpString<'data>),

    Bool(Bool),
    Null(Null),
    Integer(Integer<'data>),

    ExplicitTag(ExplicitTag<'data>),
    ApplicationTag(ApplicationTag<'data>),
}

pub type Asn1<'data> = Tlv<'data, Asn1Type<'data>>;
pub type OwnedAsn1 = Tlv<'static, Asn1Type<'static>>;

pub type OwnedAsn1Type = Asn1Type<'static>;

impl Asn1Type<'_> {
    pub fn to_owned(&self) -> OwnedAsn1Type {
        match self {
            Asn1Type::Sequence(s) => Asn1Type::Sequence(s.to_owned()),
            Asn1Type::OctetString(o) => Asn1Type::OctetString(o.to_owned()),
            Asn1Type::Utf8String(u) => Asn1Type::Utf8String(u.to_owned()),
            Asn1Type::BitString(b) => Asn1Type::BitString(b.to_owned()),
            Asn1Type::Bool(b) => Asn1Type::Bool(b.clone()),
            Asn1Type::Null(n) => Asn1Type::Null(n.clone()),
            Asn1Type::Integer(i) => Asn1Type::Integer(i.to_owned()),
            Asn1Type::ExplicitTag(e) => Asn1Type::ExplicitTag(e.to_owned()),
            Asn1Type::ApplicationTag(a) => Asn1Type::ApplicationTag(a.to_owned()),
            Asn1Type::BmpString(b) => Asn1Type::BmpString(b.to_owned()),
        }
    }
}

impl<'data> Asn1ValueDecoder<'data> for Asn1Type<'data> {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        if OctetString::compare_tags(tag) {
            Ok(Asn1Type::OctetString(OctetString::decode(tag, reader)?))
        } else if Utf8String::compare_tags(tag) {
            Ok(Asn1Type::Utf8String(Utf8String::decode(tag, reader)?))
        } else if Sequence::compare_tags(tag) {
            Ok(Asn1Type::Sequence(Sequence::decode(tag, reader)?))
        } else if BitString::compare_tags(tag) {
            Ok(Asn1Type::BitString(BitString::decode(tag, reader)?))
        } else if BmpString::compare_tags(tag) {
            Ok(Asn1Type::BmpString(BmpString::decode(tag, reader)?))
        } else if Bool::compare_tags(tag) {
            Ok(Asn1Type::Bool(Bool::decode(tag, reader)?))
        } else if Integer::compare_tags(tag) {
            Ok(Asn1Type::Integer(Integer::decode(tag, reader)?))
        } else if ExplicitTag::compare_tags(tag) {
            Ok(Asn1Type::ExplicitTag(ExplicitTag::decode(tag, reader)?))
        } else if ApplicationTag::compare_tags(tag) {
            Ok(Asn1Type::ApplicationTag(ApplicationTag::decode(tag, reader)?))
        } else if Null::compare_tags(tag) {
            Ok(Asn1Type::Null(Null::decode(tag, reader)?))
        } else {
            Err(Error::from("Invalid data"))
        }
    }

    fn compare_tags(_tag: Tag) -> bool {
        true
    }
}

impl Asn1Encoder for Asn1Type<'_> {
    fn needed_buf_size(&self) -> usize {
        match self {
            Asn1Type::OctetString(octet) => octet.needed_buf_size(),
            Asn1Type::Utf8String(utf8) => utf8.needed_buf_size(),
            Asn1Type::Sequence(sequence) => sequence.needed_buf_size(),
            Asn1Type::BitString(bit) => bit.needed_buf_size(),
            Asn1Type::BmpString(bmp) => bmp.needed_buf_size(),
            Asn1Type::Bool(boolean) => boolean.needed_buf_size(),
            Asn1Type::Integer(integer) => integer.needed_buf_size(),
            Asn1Type::ExplicitTag(e) => e.needed_buf_size(),
            Asn1Type::ApplicationTag(a) => a.needed_buf_size(),
            Asn1Type::Null(n) => n.needed_buf_size(),
        }
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        match self {
            Asn1Type::OctetString(octet) => octet.encode(writer),
            Asn1Type::Utf8String(utf8) => utf8.encode(writer),
            Asn1Type::Sequence(sequence) => sequence.encode(writer),
            Asn1Type::BitString(bit) => bit.encode(writer),
            Asn1Type::BmpString(bmp) => bmp.encode(writer),
            Asn1Type::Bool(boolean) => boolean.encode(writer),
            Asn1Type::Integer(integer) => integer.encode(writer),
            Asn1Type::ExplicitTag(e) => e.encode(writer),
            Asn1Type::ApplicationTag(a) => a.encode(writer),
            Asn1Type::Null(n) => n.encode(writer),
        }
    }
}

/// Information about raw data of the asn1 entity
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
