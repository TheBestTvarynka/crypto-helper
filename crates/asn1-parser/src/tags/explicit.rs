use alloc::borrow::Cow;
use alloc::boxed::Box;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Error, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitTag<'data> {
    id: u64,
    tag: u8,
    inner: Asn1<'data>,
}

pub type OwnedExplicitTag = ExplicitTag<'static>;

impl<'data> ExplicitTag<'data> {
    pub fn new(id: u64, tag: u8, inner: Asn1<'data>) -> Self {
        Self {
            id,
            tag: tag & 0x1f | 0xa0,
            inner,
        }
    }

    pub fn tag_number(&self) -> u8 {
        self.tag & 0x1f
    }

    pub fn inner(&self) -> &Asn1<'data> {
        &self.inner
    }

    pub fn clear_raw_data(&mut self) -> &mut Self {
        self.inner.clear_raw_data();
        self
    }

    pub fn to_owned(&self) -> OwnedExplicitTag {
        OwnedExplicitTag {
            id: self.id,
            tag: self.tag,
            inner: self.inner.to_owned(),
        }
    }
}

impl Asn1Entity for ExplicitTag<'_> {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<'data> Asn1Decoder<'data> for ExplicitTag<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        let raw_tag = tag.0;

        raw_tag & 0xc0 == 0x80 && raw_tag & 0x20 == 0x20
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag = reader.read_byte()?;

        if !Self::compare_tags(&Tag(tag)) {
            return Err(Error::from("Invalid explicit tag"));
        }

        let (len, _len_range) = read_len(reader)?;

        let inner = Asn1Type::decode_asn1(reader)?;

        if len != inner.raw_data.raw_bytes().len() {
            return Err(Error::from(
                "Invalid explicit tag len. Inner entity raw data len is not the same as explicit tag len.",
            ));
        }

        Ok(Self {
            id: reader.next_id(),
            tag,
            inner,
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        let tag = reader.read_byte()?;

        if !Self::compare_tags(&Tag(tag)) {
            return Err(Error::from("Invalid explicit tag"));
        }

        let (len, len_range) = read_len(reader)?;

        let inner = Asn1Type::decode_asn1(reader)?;

        if len != inner.raw_data.raw_bytes().len() {
            return Err(Error::from(
                "Invalid explicit tag len. Inner entity raw data len is not the same as explicit tag len.",
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
            asn1_type: Box::new(Asn1Type::ExplicitTag(Self {
                id: reader.next_id(),
                tag,
                inner,
            })),
        })
    }
}

impl Asn1Encoder for ExplicitTag<'_> {
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::dbg;

    use crate::{Asn1Decoder, Asn1Type};

    #[test]
    fn example() {
        let raw = [
            48, 50, 161, 17, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97, 162, 9,
            12, 7, 113, 107, 97, 116, 105, 111, 110, 163, 18, 4, 16, 252, 179, 92, 152, 40, 255, 170, 90, 80, 236, 156,
            221, 80, 86, 181, 110,
        ];

        let asn1 = Asn1Type::decode_asn1_buff(&raw).unwrap();

        dbg!("{:?}", asn1);
    }
}
