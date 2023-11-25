use alloc::borrow::Cow;
use alloc::boxed::Box;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Error, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationTag<'data> {
    tag: u8,
    inner: Asn1<'data>,
}

pub type OwnedApplicationTag = ApplicationTag<'static>;

impl<'data> ApplicationTag<'data> {
    pub fn new(tag: u8, inner: Asn1<'data>) -> Self {
        Self {
            tag: tag & 0x1f | 0x60,
            inner,
        }
    }

    pub fn clear_raw_data(&mut self) -> &mut Self {
        self.inner.clear_raw_data();
        self
    }

    pub fn to_owned(&self) -> OwnedApplicationTag {
        OwnedApplicationTag {
            tag: self.tag,
            inner: self.inner.to_owned(),
        }
    }
}

impl Asn1Entity for ApplicationTag<'_> {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1Decoder<'data> for ApplicationTag<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        let raw_tag = tag.0;

        raw_tag & 0xc0 == 0x40 && raw_tag & 0x20 == 0x20
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag = reader.read_byte()?;

        if !Self::compare_tags(&Tag(tag)) {
            return Err(Error::from("Invalid application tag"));
        }

        let (len, _len_range) = read_len(reader)?;

        let inner = Asn1Type::decode_asn1(reader)?;

        if len != inner.raw_data.raw_bytes().len() {
            return Err(Error::from(
                "Invalid application tag len. Inner entity raw data len is not the same as explicit tag len.",
            ));
        }

        Ok(Self { tag, inner })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        let tag = reader.read_byte()?;

        if !Self::compare_tags(&Tag(tag)) {
            return Err(Error::from("Invalid application tag"));
        }

        let (len, len_range) = read_len(reader)?;

        let inner = Asn1Type::decode_asn1(reader)?;

        if len != inner.raw_data.raw_bytes().len() {
            return Err(Error::from(
                "Invalid applicationTag tag len. Inner entity raw data len is not the same as explicit tag len.",
            ));
        }

        let inner_data_range = inner.raw_data.data_range();

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(tag_position..inner_data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: inner.raw_data.tag_position()..inner_data_range.end,
            },
            asn1_type: Box::new(Asn1Type::ApplicationTag(Self { tag, inner })),
        })
    }
}

impl Asn1Encoder for ApplicationTag<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.inner.asn1().needed_buf_size();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(self.tag)?;

        let data_len = self.inner.asn1().needed_buf_size();
        write_len(data_len, writer)?;

        self.inner.asn1().encode(writer)
    }
}
