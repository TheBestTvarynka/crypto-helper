use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitTag {
    tag: u8,
    inner: Vec<Asn1>,
}

impl ExplicitTag {
    pub fn new(tag: u8, inner: Vec<Asn1>) -> Self {
        Self {
            tag: tag & 0x1f | 0xa0,
            inner,
        }
    }

    pub fn tag_number(&self) -> u8 {
        self.tag & 0x1f
    }

    pub fn inner(&self) -> &[Asn1] {
        &self.inner
    }
}

impl Taggable for ExplicitTag {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1ValueDecoder<'data> for ExplicitTag {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let mut inner = Vec::new();

        while !reader.empty() {
            inner.push(Asn1::decode(reader)?);
        }

        Ok(Self { tag: tag.0, inner })
    }

    fn compare_tags(tag: Tag) -> bool {
        tag.is_context_specific() && tag.is_constructed()
    }
}

impl Asn1Encoder for ExplicitTag {
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

impl MetaInfo for ExplicitTag {
    fn clear_meta(&mut self) {
        self.inner.iter_mut().for_each(|f| f.clear_meta())
    }
}
