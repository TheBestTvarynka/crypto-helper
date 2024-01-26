use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, Error, MetaInfo, Tag, Taggable};

/// [BitString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bitstring.html)
///
/// ASN.1 BIT STRING type values are arbitrary length strings of bits.
/// A BIT STRING value doesn't need to be an even multiple of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitString<'data> {
    octets: Cow<'data, [u8]>,
    inner: Option<Box<Asn1<'data>>>,
}

pub type OwnedBitString = BitString<'static>;

impl BitString<'_> {
    pub const TAG: Tag = Tag(3);

    /// Returns inner bits
    pub fn raw_bits(&self) -> &[u8] {
        self.octets.as_ref()
    }

    pub fn inner(&self) -> Option<&Asn1<'_>> {
        self.inner.as_ref().map(|i| i.as_ref())
    }

    pub fn bits_amount(&self) -> usize {
        (self.octets.as_ref().len() - 1) * 8 - usize::from(self.octets.as_ref()[0])
    }

    /// Creates a new [BitString] from amount of bits and actual bits buffer
    pub fn from_raw_vec(bits_amount: usize, mut bits: Vec<u8>) -> Asn1Result<OwnedBitString> {
        let all_bits_amount = bits.len() * 8;

        if bits_amount > all_bits_amount {
            return Err(Error::from("Too many bits"));
        }

        if all_bits_amount - bits_amount >= 8 {
            return Err(Error::from("Too many unused bits"));
        }

        let unused_bits: u8 = (all_bits_amount - bits_amount).try_into()?;
        bits.insert(0, unused_bits);

        let inner = if bits.len() > 0 {
            Asn1::decode_buff(&bits[1..]).ok().map(|mut asn1| {
                asn1.clear_meta();
                Box::new(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned()))
            })
        } else {
            None
        };

        Ok(BitString {
            octets: Cow::Owned(bits),
            inner,
        })
    }

    /// Returns owned version of the [BitString]
    pub fn to_owned(&self) -> OwnedBitString {
        BitString {
            octets: self.octets.to_vec().into(),
            inner: self
                .inner
                .as_ref()
                .map(|inner| Box::new(inner.to_owned_with_asn1(inner.inner_asn1().to_owned()))),
        }
    }
}

// we assume here that firs vector byte contains amount of unused bytes
impl From<Vec<u8>> for BitString<'_> {
    fn from(data: Vec<u8>) -> Self {
        let inner = Asn1::decode_buff(&data)
            .ok()
            .map(|asn1| Box::new(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned())));
        Self {
            octets: Cow::Owned(data),
            inner,
        }
    }
}

impl Taggable for BitString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for BitString<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.read_remaining();

        let inner = if data.len() > 0 {
            let mut inner_reader = Reader::new(&data[1..]);
            inner_reader.set_next_id(reader.next_id());
            inner_reader.set_offset(reader.full_offset() - data.len());
            let mut inner = Asn1::decode(&mut inner_reader).ok().map(Box::new);

            if !inner_reader.empty() && inner.is_some() {
                inner = None;
            }

            reader.set_next_id(inner_reader.next_id());

            inner
        } else {
            None
        };

        Ok(Self {
            octets: Cow::Borrowed(data),
            inner,
        })
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for BitString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.octets.len();
        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.octets.len(), writer)?;
        writer.write_slice(&self.octets)
    }
}
