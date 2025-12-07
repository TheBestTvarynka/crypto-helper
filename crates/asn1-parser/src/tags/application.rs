use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationTag {
    tag: u8,
    inner: Vec<Asn1>,
}

impl ApplicationTag {
    pub fn new(tag: u8, inner: Vec<Asn1>) -> Self {
        Self {
            tag: tag & 0x1f | 0x60,
            inner,
        }
    }

    pub fn set_tag_number(&mut self, tag: u8) {
        self.tag = tag & 0x1f | 0x60;
    }

    pub fn tag_number(&self) -> u8 {
        self.tag & 0x1f
    }

    pub fn inner(&self) -> &[Asn1] {
        &self.inner
    }

    pub fn fields_mut_vec(&mut self) -> &mut Vec<Asn1> {
        &mut self.inner
    }
}

impl Taggable for ApplicationTag {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1ValueDecoder<'data> for ApplicationTag {
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

impl Asn1Encoder for ApplicationTag {
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

impl MetaInfo for ApplicationTag {
    fn clear_meta(&mut self) {
        self.inner.iter_mut().for_each(|f| f.clear_meta())
    }
}
