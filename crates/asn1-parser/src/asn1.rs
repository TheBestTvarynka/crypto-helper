use alloc::borrow::Cow;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    ApplicationTag, Asn1Encoder, Asn1Result, Asn1ValueDecoder, BitString, BmpString, Bool, Enumerated, Error, ExplicitTag, GeneralString, GeneralizedTime, IA5String, ImplicitTag, Integer, IntoMutable, MetaInfo, Mutable, Null, NumericString, ObjectIdentifier, OctetString, PrintableString, Sequence, Set, Tag, Taggable, Tlv, UtcTime, Utf8String, VisibleString
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Type<'data> {
    Sequence(Sequence<'data>),
    Set(Set<'data>),

    OctetString(OctetString<'data>),
    Utf8String(Utf8String<'data>),
    BitString(BitString<'data>),
    BmpString(BmpString<'data>),
    IA5String(IA5String<'data>),
    PrintableString(PrintableString<'data>),
    GeneralString(GeneralString<'data>),
    NumericString(NumericString<'data>),
    VisibleString(VisibleString<'data>),

    UtcTime(UtcTime),
    GeneralizedTime(GeneralizedTime),

    Bool(Bool),
    Null(Null),
    Integer(Integer<'data>),
    Enumerated(Enumerated<'data>),
    ObjectIdentifier(ObjectIdentifier),

    ExplicitTag(ExplicitTag<'data>),
    ImplicitTag(ImplicitTag<'data>),
    ApplicationTag(ApplicationTag<'data>),

    Mutable(Mutable<OwnedAsn1Type>),
}

pub type Asn1<'data> = Tlv<'data, Asn1Type<'data>>;
pub type OwnedAsn1 = Tlv<'static, Asn1Type<'static>>;

pub type OwnedAsn1Type = Asn1Type<'static>;

impl Asn1Type<'_> {
    pub fn to_owned(&self) -> OwnedAsn1Type {
        match self {
            Asn1Type::Sequence(s) => Asn1Type::Sequence(s.to_owned()),
            Asn1Type::Set(s) => Asn1Type::Set(s.to_owned()),
            Asn1Type::OctetString(o) => Asn1Type::OctetString(o.to_owned()),
            Asn1Type::Utf8String(u) => Asn1Type::Utf8String(u.to_owned()),
            Asn1Type::BitString(b) => Asn1Type::BitString(b.to_owned()),
            Asn1Type::IA5String(i) => Asn1Type::IA5String(i.to_owned()),
            Asn1Type::PrintableString(p) => Asn1Type::PrintableString(p.to_owned()),
            Asn1Type::GeneralString(g) => Asn1Type::GeneralString(g.to_owned()),
            Asn1Type::NumericString(n) => Asn1Type::NumericString(n.to_owned()),
            Asn1Type::VisibleString(n) => Asn1Type::VisibleString(n.to_owned()),
            Asn1Type::Bool(b) => Asn1Type::Bool(b.clone()),
            Asn1Type::Null(n) => Asn1Type::Null(n.clone()),
            Asn1Type::Integer(i) => Asn1Type::Integer(i.to_owned()),
            Asn1Type::Enumerated(e) => Asn1Type::Enumerated(e.to_owned()),
            Asn1Type::ObjectIdentifier(o) => Asn1Type::ObjectIdentifier(o.clone()),
            Asn1Type::ExplicitTag(e) => Asn1Type::ExplicitTag(e.to_owned()),
            Asn1Type::ImplicitTag(i) => Asn1Type::ImplicitTag(i.to_owned()),
            Asn1Type::ApplicationTag(a) => Asn1Type::ApplicationTag(a.to_owned()),
            Asn1Type::BmpString(b) => Asn1Type::BmpString(b.to_owned()),
            Asn1Type::UtcTime(u) => Asn1Type::UtcTime(u.clone()),
            Asn1Type::GeneralizedTime(u) => Asn1Type::GeneralizedTime(u.clone()),
            Asn1Type::Mutable(m) => Asn1Type::Mutable(m.clone()),
        }
    }
}

impl IntoMutable<OwnedAsn1Type> for Asn1Type<'_> {
    fn into_mutable(self) -> Mutable<OwnedAsn1Type> {
        //
    }
}

impl Taggable for Asn1Type<'_> {
    fn tag(&self) -> Tag {
        match self {
            Asn1Type::Sequence(s) => s.tag(),
            Asn1Type::Set(s) => s.tag(),
            Asn1Type::OctetString(o) => o.tag(),
            Asn1Type::Utf8String(u) => u.tag(),
            Asn1Type::BitString(b) => b.tag(),
            Asn1Type::BmpString(b) => b.tag(),
            Asn1Type::IA5String(i) => i.tag(),
            Asn1Type::PrintableString(p) => p.tag(),
            Asn1Type::GeneralString(g) => g.tag(),
            Asn1Type::NumericString(g) => g.tag(),
            Asn1Type::VisibleString(g) => g.tag(),
            Asn1Type::Bool(b) => b.tag(),
            Asn1Type::Null(n) => n.tag(),
            Asn1Type::Integer(i) => i.tag(),
            Asn1Type::Enumerated(e) => e.tag(),
            Asn1Type::ObjectIdentifier(o) => o.tag(),
            Asn1Type::ExplicitTag(e) => e.tag(),
            Asn1Type::ImplicitTag(i) => i.tag(),
            Asn1Type::ApplicationTag(a) => a.tag(),
            Asn1Type::UtcTime(u) => u.tag(),
            Asn1Type::GeneralizedTime(u) => u.tag(),
            Asn1Type::Mutable(m) => m.tag(),
        }
    }
}

impl<'data> Asn1ValueDecoder<'data> for Asn1Type<'data> {
    #[instrument(level = "debug", ret)]
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        debug!(?tag);

        decode_asn1!(
            OctetString,
            Utf8String,
            Sequence,
            Set,
            BitString,
            BmpString,
            IA5String,
            PrintableString,
            GeneralString,
            NumericString,
            VisibleString,
            Bool,
            Integer,
            Enumerated,
            ObjectIdentifier,
            ExplicitTag,
            ImplicitTag,
            ApplicationTag,
            Null,
            UtcTime,
            GeneralizedTime;
            in tag, reader
        );

        Err(Error::from("Invalid or unsupported asn1 tag"))
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
            Asn1Type::Set(set) => set.needed_buf_size(),
            Asn1Type::BitString(bit) => bit.needed_buf_size(),
            Asn1Type::BmpString(bmp) => bmp.needed_buf_size(),
            Asn1Type::IA5String(i) => i.needed_buf_size(),
            Asn1Type::PrintableString(p) => p.needed_buf_size(),
            Asn1Type::GeneralString(g) => g.needed_buf_size(),
            Asn1Type::NumericString(g) => g.needed_buf_size(),
            Asn1Type::VisibleString(g) => g.needed_buf_size(),
            Asn1Type::Bool(boolean) => boolean.needed_buf_size(),
            Asn1Type::Integer(integer) => integer.needed_buf_size(),
            Asn1Type::Enumerated(enumerated) => enumerated.needed_buf_size(),
            Asn1Type::ObjectIdentifier(object_identifier) => object_identifier.needed_buf_size(),
            Asn1Type::ExplicitTag(e) => e.needed_buf_size(),
            Asn1Type::ImplicitTag(i) => i.needed_buf_size(),
            Asn1Type::ApplicationTag(a) => a.needed_buf_size(),
            Asn1Type::Null(n) => n.needed_buf_size(),
            Asn1Type::UtcTime(u) => u.needed_buf_size(),
            Asn1Type::GeneralizedTime(u) => u.needed_buf_size(),
            Asn1Type::Mutable(m) => m.needed_buf_size(),
        }
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        match self {
            Asn1Type::OctetString(octet) => octet.encode(writer),
            Asn1Type::Utf8String(utf8) => utf8.encode(writer),
            Asn1Type::Sequence(sequence) => sequence.encode(writer),
            Asn1Type::Set(set) => set.encode(writer),
            Asn1Type::BitString(bit) => bit.encode(writer),
            Asn1Type::BmpString(bmp) => bmp.encode(writer),
            Asn1Type::IA5String(ia5) => ia5.encode(writer),
            Asn1Type::PrintableString(printable) => printable.encode(writer),
            Asn1Type::GeneralString(general) => general.encode(writer),
            Asn1Type::NumericString(numeric) => numeric.encode(writer),
            Asn1Type::VisibleString(numeric) => numeric.encode(writer),
            Asn1Type::Bool(boolean) => boolean.encode(writer),
            Asn1Type::Integer(integer) => integer.encode(writer),
            Asn1Type::Enumerated(enumerated) => enumerated.encode(writer),
            Asn1Type::ObjectIdentifier(object_identifier) => object_identifier.encode(writer),
            Asn1Type::ExplicitTag(e) => e.encode(writer),
            Asn1Type::ImplicitTag(i) => i.encode(writer),
            Asn1Type::ApplicationTag(a) => a.encode(writer),
            Asn1Type::Null(n) => n.encode(writer),
            Asn1Type::UtcTime(utc_time) => utc_time.encode(writer),
            Asn1Type::GeneralizedTime(generalized_time) => generalized_time.encode(writer),
            Asn1Type::Mutable(m) => m.encode(writer),
        }
    }
}

impl MetaInfo for Asn1Type<'_> {
    fn clear_meta(&mut self) {
        match self {
            Asn1Type::OctetString(octet_string) => octet_string.clear_meta(),
            Asn1Type::Utf8String(_) => {}
            Asn1Type::Sequence(sequence) => sequence.clear_meta(),
            Asn1Type::Set(set) => set.clear_meta(),
            Asn1Type::BitString(_) => {}
            Asn1Type::BmpString(_) => {}
            Asn1Type::IA5String(_) => {}
            Asn1Type::PrintableString(_) => {}
            Asn1Type::GeneralString(_) => {}
            Asn1Type::NumericString(_) => {}
            Asn1Type::VisibleString(_) => {}
            Asn1Type::Bool(_) => {}
            Asn1Type::Integer(_) => {}
            Asn1Type::Enumerated(_) => {}
            Asn1Type::ObjectIdentifier(_) => {}
            Asn1Type::ExplicitTag(explicit_tag) => explicit_tag.clear_meta(),
            Asn1Type::ImplicitTag(implicit_tag) => implicit_tag.clear_meta(),
            Asn1Type::ApplicationTag(application_tag) => application_tag.clear_meta(),
            Asn1Type::Null(_) => {}
            Asn1Type::UtcTime(_) => {}
            Asn1Type::GeneralizedTime(_) => {}
            Asn1Type::Mutable(m) => m.clear_meta(),
        }
    }
}

/// Information about raw data of the asn1 entity
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawAsn1EntityData<'data> {
    /// Raw input bytes for the *current* asn1 node
    pub raw_data: Cow<'data, [u8]>,

    /// Position of the tag in the input data
    pub tag: usize,

    /// Range that corresponds to the encoded length bytes in the raw_data
    pub length: Range<usize>,

    /// Range that corresponds to the inner data in the raw_data
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
