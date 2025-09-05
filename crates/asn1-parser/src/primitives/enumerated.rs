use alloc::borrow::Cow;
use alloc::vec::Vec;

use num_bigint_dig::BigUint;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, IntoMutable, Mutable, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumerated<'data>(Cow<'data, [u8]>);

pub type OwnedEnumerated = Enumerated<'static>;

impl Enumerated<'_> {
    pub const TAG: Tag = Tag(10);

    pub fn raw_data(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn as_big_uint(&self) -> BigUint {
        BigUint::from_bytes_be(if self.0.len() > 1 {
            if self.0[0] == 0x00 { &self.0[1..] } else { &self.0 }
        } else if self.0.is_empty() {
            &[0]
        } else {
            &self.0
        })
    }

    pub fn to_owned(&self) -> OwnedEnumerated {
        Enumerated(Cow::Owned(self.0.as_ref().to_vec()))
    }
}

impl From<Vec<u8>> for OwnedEnumerated {
    fn from(bytes: Vec<u8>) -> Self {
        Self(Cow::Owned(bytes))
    }
}

impl IntoMutable<OwnedEnumerated> for Enumerated<'_> {
    fn into_mutable(self) -> Mutable<OwnedEnumerated> {
        Mutable::new(Enumerated(match self.0 {
            Cow::Owned(data) => Cow::Owned(data),
            Cow::Borrowed(data) => Cow::Owned(data.to_vec()),
        }))
    }
}

impl Taggable for Enumerated<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for Enumerated<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        Ok(Self(Cow::Borrowed(reader.remaining())))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for Enumerated<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.0.len(), writer)?;
        writer.write_slice(self.0.as_ref())
    }
}
