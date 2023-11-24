use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, BitString, BmpString, Bool, Error, ExplicitTag, Null,
    OctetString, Sequence, Tag, Utf8String,
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

    ExplicitTag(ExplicitTag<'data>),
}

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
            Asn1Type::BmpString(bmp) => bmp.tag(),
            Asn1Type::Bool(boolean) => boolean.tag(),
            Asn1Type::ExplicitTag(e) => e.tag(),
            Asn1Type::Null(n) => n.tag(),
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
        } else if BmpString::compare_tags(&tag) {
            Ok(Asn1Type::BmpString(BmpString::decode(reader)?))
        } else if Bool::compare_tags(&tag) {
            Ok(Asn1Type::Bool(Bool::decode(reader)?))
        } else if ExplicitTag::compare_tags(&tag) {
            Ok(Asn1Type::ExplicitTag(ExplicitTag::decode(reader)?))
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
        } else if ExplicitTag::compare_tags(&tag) {
            ExplicitTag::decode_asn1(reader)
        } else if Null::compare_tags(&tag) {
            Null::decode_asn1(reader)
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
            Asn1Type::ExplicitTag(e) => e.needed_buf_size(),
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
            Asn1Type::ExplicitTag(e) => e.encode(writer),
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
            48, 130, 2, 30, 4, 7, 58, 232, 40, 24, 17, 216, 176, 4, 11, 216, 44, 74, 26, 137, 109, 11, 173, 211, 185,
            135, 48, 129, 255, 3, 5, 7, 67, 253, 55, 182, 12, 48, 125, 242, 181, 151, 128, 241, 174, 168, 128, 42, 243,
            146, 170, 138, 47, 10, 123, 242, 187, 129, 138, 241, 183, 182, 166, 195, 168, 240, 146, 188, 162, 226, 128,
            174, 104, 61, 100, 103, 242, 147, 175, 190, 36, 61, 103, 96, 58, 36, 1, 1, 0, 12, 48, 127, 241, 148, 178,
            169, 61, 243, 128, 178, 143, 195, 141, 0, 0, 59, 58, 51, 58, 242, 154, 140, 138, 243, 136, 172, 139, 96,
            58, 46, 240, 159, 149, 180, 119, 97, 39, 226, 128, 174, 241, 185, 169, 191, 241, 178, 167, 141, 36, 4, 5,
            50, 178, 83, 20, 77, 4, 1, 140, 12, 58, 242, 167, 134, 180, 61, 239, 187, 191, 195, 181, 242, 159, 149,
            180, 47, 96, 242, 140, 178, 173, 226, 128, 174, 240, 159, 149, 180, 242, 189, 135, 177, 88, 194, 165, 0,
            58, 114, 209, 168, 241, 172, 148, 154, 240, 159, 149, 180, 41, 37, 242, 190, 156, 163, 235, 135, 172, 194,
            165, 1, 1, 0, 4, 12, 117, 86, 112, 180, 20, 202, 224, 28, 58, 3, 133, 90, 4, 15, 212, 214, 53, 154, 145, 2,
            117, 175, 243, 103, 181, 102, 19, 251, 188, 3, 20, 5, 221, 77, 67, 230, 172, 240, 96, 163, 227, 181, 175,
            194, 248, 31, 235, 105, 46, 230, 38, 3, 17, 5, 155, 82, 70, 117, 135, 62, 42, 165, 241, 155, 147, 173, 209,
            54, 160, 138, 4, 4, 129, 51, 94, 101, 48, 46, 12, 44, 231, 158, 163, 58, 50, 63, 96, 10, 53, 111, 51, 123,
            63, 240, 184, 179, 161, 209, 168, 123, 242, 150, 186, 157, 243, 170, 137, 140, 200, 186, 242, 155, 156,
            156, 92, 0, 27, 195, 149, 92, 242, 156, 178, 152, 3, 18, 0, 206, 190, 97, 201, 75, 125, 225, 116, 109, 226,
            236, 4, 19, 9, 7, 185, 100, 12, 57, 243, 176, 171, 184, 58, 92, 243, 166, 183, 187, 243, 132, 154, 159,
            243, 165, 189, 155, 61, 36, 46, 66, 239, 191, 189, 240, 177, 152, 141, 241, 168, 156, 143, 13, 194, 134,
            39, 240, 172, 142, 137, 46, 240, 159, 149, 180, 10, 243, 133, 180, 182, 69, 242, 128, 164, 156, 122, 1, 1,
            0, 3, 16, 1, 66, 115, 229, 233, 85, 68, 237, 69, 93, 254, 218, 104, 75, 133, 241, 4, 18, 75, 232, 138, 24,
            247, 158, 233, 154, 181, 156, 155, 252, 71, 105, 43, 215, 211, 160, 3, 23, 2, 102, 29, 9, 2, 92, 205, 26,
            162, 54, 221, 33, 80, 194, 82, 99, 110, 161, 116, 102, 123, 221, 240, 12, 61, 241, 164, 185, 138, 243, 177,
            154, 148, 243, 150, 162, 184, 209, 168, 123, 46, 240, 147, 130, 133, 63, 242, 147, 170, 174, 226, 128, 174,
            11, 241, 174, 152, 137, 27, 37, 10, 241, 164, 144, 156, 243, 148, 132, 139, 241, 150, 160, 154, 241, 169,
            185, 175, 226, 128, 174, 226, 128, 174, 0, 70, 45,
        ])
        .unwrap()
    }
}
