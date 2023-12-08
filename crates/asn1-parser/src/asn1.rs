use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    ApplicationTag, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, BitString, BmpString, Bool, Error, ExplicitTag,
    Integer, Null, OctetString, Sequence, Tag, Utf8String,
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

pub type OwnedAsn1Type = Asn1Type<'static>;

impl Asn1Type<'_> {
    pub fn clear_raw_data(&mut self) -> &mut Self {
        match self {
            Asn1Type::Sequence(s) => {
                s.clear_raw_data();
            }
            Asn1Type::OctetString(_) => {}
            Asn1Type::Utf8String(_) => {}
            Asn1Type::BitString(_) => {}
            Asn1Type::BmpString(_) => {}
            Asn1Type::Bool(_) => {}
            Asn1Type::Null(_) => {}
            Asn1Type::Integer(_) => {}
            Asn1Type::ExplicitTag(e) => {
                e.clear_raw_data();
            }
            Asn1Type::ApplicationTag(a) => {
                a.clear_raw_data();
            }
        };
        self
    }

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
    fn compare_tags(_tag: &Tag) -> bool {
        // OctetString::compare_tags(tag) || Utf8String::compare_tags(tag)
        true
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
        } else if BmpString::compare_tags(&tag) {
            Ok(Asn1Type::BmpString(BmpString::decode(reader)?))
        } else if Bool::compare_tags(&tag) {
            Ok(Asn1Type::Bool(Bool::decode(reader)?))
        } else if Integer::compare_tags(&tag) {
            Ok(Asn1Type::Integer(Integer::decode(reader)?))
        } else if ExplicitTag::compare_tags(&tag) {
            Ok(Asn1Type::ExplicitTag(ExplicitTag::decode(reader)?))
        } else if ApplicationTag::compare_tags(&tag) {
            Ok(Asn1Type::ApplicationTag(ApplicationTag::decode(reader)?))
        } else if Null::compare_tags(&tag) {
            Ok(Asn1Type::Null(Null::decode(reader)?))
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
        } else if BmpString::compare_tags(&tag) {
            BmpString::decode_asn1(reader)
        } else if Bool::compare_tags(&tag) {
            Bool::decode_asn1(reader)
        } else if Integer::compare_tags(&tag) {
            Integer::decode_asn1(reader)
        } else if ExplicitTag::compare_tags(&tag) {
            ExplicitTag::decode_asn1(reader)
        } else if ApplicationTag::compare_tags(&tag) {
            ApplicationTag::decode_asn1(reader)
        } else if Null::compare_tags(&tag) {
            Null::decode_asn1(reader)
        } else {
            Err(Error::from("Asn1Type: Invalid data"))
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
    pub fn new<'data>(raw_data: RawAsn1EntityData<'data>, asn1_type: Box<Asn1Type<'data>>) -> Asn1<'data> {
        Asn1 { raw_data, asn1_type }
    }

    pub fn raw_entity_data(&self) -> &RawAsn1EntityData<'_> {
        &self.raw_data
    }

    pub fn asn1(&self) -> &Asn1Type<'_> {
        &self.asn1_type
    }

    pub fn clear_raw_data(&mut self) -> &mut Self {
        self.raw_data = Default::default();
        self.asn1_type.clear_raw_data();
        self
    }

    pub fn to_owned(&self) -> OwnedAsn1 {
        Asn1 {
            raw_data: self.raw_data.to_owned(),
            asn1_type: Box::new((*self.asn1_type).to_owned()),
        }
    }
}

impl Default for Asn1<'_> {
    fn default() -> Self {
        // those values are just for testing purpose during development
        Asn1Type::decode_asn1_buff(&[
            48, 87, 1, 1, 255, 1, 1, 0, 160, 17, 12, 15, 84, 98, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110,
            107, 97, 161, 60, 48, 58, 5, 0, 164, 9, 4, 7, 48, 5, 160, 3, 1, 1, 255, 164, 7, 3, 5, 0, 64, 129, 0, 16,
            164, 34, 108, 32, 48, 30, 160, 2, 5, 0, 161, 24, 30, 22, 0, 67, 0, 101, 0, 114, 0, 116, 0, 105, 0, 102, 0,
            105, 0, 99, 0, 97, 0, 116, 0, 101,
        ])
        .unwrap()
    }
}
