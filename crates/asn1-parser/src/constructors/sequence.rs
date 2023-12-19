use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

/// [ASN.1 SEQUENCE](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/sequence.html)
///
/// In ASN.1, an ordered list of elements (or components) comprises a SEQUENCE.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sequence<'data>(Vec<Asn1<'data>>);

pub type OwnedSequence = Sequence<'static>;

impl Sequence<'_> {
    pub const TAG: Tag = Tag(0x30);

    pub fn new(fields: Vec<Asn1>) -> Sequence {
        Sequence(fields)
    }

    /// Returns [Sequence] fields
    pub fn fields(&self) -> &[Asn1<'_>] {
        &self.0
    }

    /// Returns owned version of the [Sequence]
    pub fn to_owned(&self) -> OwnedSequence {
        Sequence(
            self.0
                .iter()
                .map(|f| f.to_owned_with_asn1(f.inner_asn1().to_owned()))
                .collect(),
        )
    }
}

impl<'data> From<Vec<Asn1<'data>>> for Sequence<'data> {
    fn from(fields: Vec<Asn1<'data>>) -> Self {
        Self(fields)
    }
}

impl Taggable for Sequence<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for Sequence<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.iter().map(|f| f.needed_buf_size()).sum();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;

        let data_len = self.0.iter().map(|f| f.needed_buf_size()).sum();
        write_len(data_len, writer)?;

        self.0.iter().try_for_each(|f| f.encode(writer))
    }
}

impl<'data> Asn1ValueDecoder<'data> for Sequence<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let mut fields = Vec::new();

        while !reader.empty() {
            fields.push(Asn1::decode(reader)?);
        }

        Ok(Self(fields))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}
