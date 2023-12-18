use alloc::borrow::Cow;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    ApplicationTag, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, BitString, BmpString, Bool, Error, ExplicitTag,
    Integer, Null, OctetString, Sequence, Tag, Tlv, Utf8String,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    Sequence(Tlv<'data, Sequence<'data>>),
    OctetString(Tlv<'data, OctetString<'data>>),
    Utf8String(Tlv<'data, Utf8String<'data>>),
    BitString(Tlv<'data, BitString<'data>>),
    BmpString(Tlv<'data, BmpString<'data>>),

    Bool(Tlv<'data, Bool>),
    Null(Tlv<'data, Null>),
    Integer(Tlv<'data, Integer<'data>>),
    ExplicitTag(Tlv<'data, ExplicitTag<'data>>),
    ApplicationTag(Tlv<'data, ApplicationTag<'data>>),
}

pub type OwnedAsn1Type = Asn1Type<'static>;

impl Asn1Type<'_> {
    pub fn clear_raw_data(&mut self) -> &mut Self {
        match self {
            Asn1Type::Sequence(_) => {}
            Asn1Type::OctetString(_) => {}
            Asn1Type::Utf8String(_) => {}
            Asn1Type::BitString(_) => {}
            Asn1Type::BmpString(_) => {}
            Asn1Type::Bool(_) => {}
            Asn1Type::Null(_) => {}
            Asn1Type::Integer(_) => {}
            Asn1Type::ExplicitTag(_) => {}
            Asn1Type::ApplicationTag(_) => {}
        };

        self
    }

    pub fn to_owned(&self) -> OwnedAsn1Type {
        match self {
            Asn1Type::Sequence(s) => Asn1Type::Sequence(s.to_owned_with_asn1(s.inner_asn1().to_owned())),
            Asn1Type::OctetString(o) => Asn1Type::OctetString(o.to_owned_with_asn1(o.inner_asn1().to_owned())),
            Asn1Type::Utf8String(u) => Asn1Type::Utf8String(u.to_owned_with_asn1(u.inner_asn1().to_owned())),
            Asn1Type::BitString(b) => Asn1Type::BitString(b.to_owned_with_asn1(b.inner_asn1().to_owned())),
            Asn1Type::BmpString(b) => Asn1Type::BmpString(b.to_owned_with_asn1(b.inner_asn1().to_owned())),
            Asn1Type::Bool(b) => Asn1Type::Bool(b.to_owned_with_asn1(b.inner_asn1().clone())),
            Asn1Type::Null(n) => Asn1Type::Null(n.to_owned_with_asn1(Null)),
            Asn1Type::Integer(i) => Asn1Type::Integer(i.to_owned_with_asn1(i.inner_asn1().to_owned())),
            Asn1Type::ExplicitTag(e) => Asn1Type::ExplicitTag(e.to_owned_with_asn1(e.inner_asn1().to_owned())),
            Asn1Type::ApplicationTag(a) => Asn1Type::ApplicationTag(a.to_owned_with_asn1(a.inner_asn1().to_owned())),
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
            Asn1Type::BmpString(bmp) => bmp.tag(),
            Asn1Type::Bool(boolean) => boolean.tag(),
            Asn1Type::Integer(integer) => integer.tag(),
            Asn1Type::ExplicitTag(e) => e.tag(),
            Asn1Type::ApplicationTag(a) => a.tag(),
            Asn1Type::Null(n) => n.tag(),
        }
    }

    fn id(&self) -> u64 {
        match self {
            Asn1Type::OctetString(octet) => octet.id(),
            Asn1Type::Utf8String(utf8) => utf8.id(),
            Asn1Type::Sequence(sequence) => sequence.id(),
            Asn1Type::BitString(bit) => bit.id(),
            Asn1Type::BmpString(bmp) => bmp.id(),
            Asn1Type::Bool(boolean) => boolean.id(),
            Asn1Type::Integer(integer) => integer.id(),
            Asn1Type::ExplicitTag(e) => e.id(),
            Asn1Type::ApplicationTag(a) => a.id(),
            Asn1Type::Null(n) => n.id(),
        }
    }
}

impl<'data> Asn1Decoder<'data> for Asn1Type<'data> {
    fn compare_tags(_tag: Tag) -> bool {
        true
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag = Tag(reader.peek_byte()?);

        if Tlv::<OctetString>::compare_tags(tag) {
            Ok(Asn1Type::OctetString(Tlv::decode(reader)?))
        } else if Tlv::<Utf8String>::compare_tags(tag) {
            Ok(Asn1Type::Utf8String(Tlv::decode(reader)?))
        } else if Tlv::<Sequence>::compare_tags(tag) {
            Ok(Asn1Type::Sequence(Tlv::decode(reader)?))
        } else if Tlv::<BitString>::compare_tags(tag) {
            Ok(Asn1Type::BitString(Tlv::decode(reader)?))
        } else if Tlv::<BmpString>::compare_tags(tag) {
            Ok(Asn1Type::BmpString(Tlv::decode(reader)?))
        } else if Tlv::<Bool>::compare_tags(tag) {
            Ok(Asn1Type::Bool(Tlv::decode(reader)?))
        } else if Tlv::<Integer>::compare_tags(tag) {
            Ok(Asn1Type::Integer(Tlv::decode(reader)?))
        } else if Tlv::<ExplicitTag>::compare_tags(tag) {
            Ok(Asn1Type::ExplicitTag(Tlv::decode(reader)?))
        } else if Tlv::<ApplicationTag>::compare_tags(tag) {
            Ok(Asn1Type::ApplicationTag(Tlv::decode(reader)?))
        } else if Tlv::<Null>::compare_tags(tag) {
            Ok(Asn1Type::Null(Tlv::decode(reader)?))
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
