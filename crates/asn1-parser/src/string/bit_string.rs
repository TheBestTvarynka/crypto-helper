use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Error, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitString<'data> {
    bits: Cow<'data, [u8]>,
}

pub type OwnedBitString = BitString<'static>;

impl BitString<'_> {
    pub const TAG: Tag = Tag(3);

    pub fn from_raw_vec(bits_amount: usize, mut bits: Vec<u8>) -> Asn1Result<Self> {
        let all_bits_amount = bits.len() * 8;

        if bits_amount > all_bits_amount {
            return Err(Error::from("Too many bits"));
        }

        if all_bits_amount - bits_amount >= 8 {
            return Err(Error::from("Too many unused bits"));
        }

        let unused_bits: u8 = (all_bits_amount - bits_amount).try_into()?;
        bits.insert(0, unused_bits);

        Ok(Self { bits: Cow::Owned(bits) })
    }
}

// we assume here that firs vector byte contains amount of unused bytes
impl From<Vec<u8>> for BitString<'_> {
    fn from(bits: Vec<u8>) -> Self {
        Self { bits: Cow::Owned(bits) }
    }
}

impl Asn1Entity for BitString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1Decoder<'data> for BitString<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        Self::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let data = reader.read(len)?;

        Ok(Self {
            bits: Cow::Borrowed(data),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: Cow::Borrowed(reader.data_in_range(tag_position..data_range.end)?),
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Box::new(Asn1Type::BitString(Self {
                bits: Cow::Borrowed(data),
            })),
        })
    }
}

impl Asn1Encoder for BitString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.bits.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.bits.len(), writer)?;
        writer.write_slice(&self.bits)
    }
}
