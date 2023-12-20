use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Error, Tag, Taggable};

/// [BitString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bitstring.html)
///
/// ASN.1 BIT STRING type values are arbitrary length strings of bits.
/// A BIT STRING value doesn't need to be an even multiple of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitString<'data>(Cow<'data, [u8]>);

pub type OwnedBitString = BitString<'static>;

impl BitString<'_> {
    pub const TAG: Tag = Tag(3);

    /// Returns inner bits
    pub fn raw_bits(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn bits_amount(&self) -> usize {
        (self.0.as_ref().len() - 1) * 8 - usize::from(self.0.as_ref()[0])
    }

    /// Creates a new [BitString] from amount of bits and actual bits buffer
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

        Ok(Self(Cow::Owned(bits)))
    }

    /// Returns owned version of the [BitString]
    pub fn to_owned(&self) -> OwnedBitString {
        BitString(self.0.to_vec().into())
    }
}

// we assume here that firs vector byte contains amount of unused bytes
impl From<Vec<u8>> for BitString<'_> {
    fn from(bits: Vec<u8>) -> Self {
        Self(Cow::Owned(bits))
    }
}

impl Taggable for BitString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for BitString<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        Ok(Self(Cow::Borrowed(reader.remaining())))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for BitString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.len();
        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.0.len(), writer)?;
        writer.write_slice(&self.0)
    }
}
