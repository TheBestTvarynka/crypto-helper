use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, RawAsn1EntityData, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer<'data> {
    id: u64,
    bytes: Cow<'data, [u8]>,
}

pub type OwnedInteger = Integer<'static>;

impl Integer<'_> {
    pub const TAG: Tag = Tag(2);

    pub fn raw_data(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn to_owned(&self) -> OwnedInteger {
        OwnedInteger {
            id: self.id,
            bytes: Cow::Owned(self.bytes.as_ref().to_vec()),
        }
    }
}

impl From<Vec<u8>> for OwnedInteger {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            id: 0,
            bytes: Cow::Owned(bytes),
        }
    }
}

impl Asn1Entity for Integer<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<'data> Asn1Decoder<'data> for Integer<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        Integer::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;
        let data = reader.read(len)?;

        Ok(Self {
            id: reader.next_id(),
            bytes: Cow::Borrowed(data),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.full_offset();
        let data_start = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(data_start..data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::Integer(Self {
                id: reader.next_id(),
                bytes: Cow::Borrowed(data),
            })),
        })
    }
}

impl Asn1Encoder for Integer<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.bytes.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.bytes.len(), writer)?;
        writer.write_slice(self.bytes.as_ref())
    }
}
