use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationTag<'data> {
    tag: u8,
    inner: Vec<Asn1<'data>>,
}

pub type OwnedApplicationTag = ApplicationTag<'static>;

impl<'data> ApplicationTag<'data> {
    pub fn new(tag: u8, inner: Vec<Asn1<'data>>) -> Self {
        Self {
            tag: tag & 0x1f | 0x60,
            inner,
        }
    }

    pub fn tag_number(&self) -> u8 {
        self.tag & 0x1f
    }

    pub fn inner(&self) -> &[Asn1<'data>] {
        &self.inner
    }

    pub fn to_owned(&self) -> OwnedApplicationTag {
        OwnedApplicationTag {
            tag: self.tag,
            inner: self
                .inner
                .iter()
                .map(|f| f.to_owned_with_asn1(f.inner_asn1().to_owned()))
                .collect(),
        }
    }
}

impl Taggable for ApplicationTag<'_> {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1ValueDecoder<'data> for ApplicationTag<'data> {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let mut inner = Vec::new();

        while !reader.empty() {
            inner.push(Asn1::decode(reader)?);
        }

        Ok(Self { tag: tag.0, inner })
    }

    fn compare_tags(tag: Tag) -> bool {
        tag.is_application() && tag.is_constructed()
    }
}

impl Asn1Encoder for ApplicationTag<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.inner.iter().map(|f| f.needed_buf_size()).sum();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(self.tag)?;

        let data_len = self.inner.iter().map(|f| f.needed_buf_size()).sum();
        write_len(data_len, writer)?;

        self.inner.iter().try_for_each(|f| f.encode(writer))
    }
}

impl MetaInfo for ApplicationTag<'_> {
    fn clear_meta(&mut self) {
        self.inner.iter_mut().for_each(|f| f.clear_meta())
    }
}
