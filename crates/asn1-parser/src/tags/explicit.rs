use alloc::boxed::Box;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitTag<'data> {
    tag: u8,
    inner: Box<Asn1<'data>>,
}

pub type OwnedExplicitTag = ExplicitTag<'static>;

impl<'data> ExplicitTag<'data> {
    pub fn new(tag: u8, inner: Box<Asn1<'data>>) -> Self {
        Self {
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

    pub fn to_owned(&self) -> OwnedExplicitTag {
        OwnedExplicitTag {
            tag: self.tag,
            inner: Box::new(self.inner.to_owned_with_asn1(self.inner.inner_asn1().to_owned())),
        }
    }
}

impl Taggable for ExplicitTag<'_> {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1ValueDecoder<'data> for ExplicitTag<'data> {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let inner = Box::new(Asn1::decode(reader)?);

        if !reader.empty() {
            return Err("explicit tag inner data contains leftovers".into());
        }

        Ok(Self { tag: tag.0, inner })
    }

    fn compare_tags(tag: Tag) -> bool {
        tag.is_context_specific() && tag.is_constructed()
    }
}

impl Asn1Encoder for ExplicitTag<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.inner.needed_buf_size();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(self.tag)?;

        let data_len = self.inner.needed_buf_size();
        write_len(data_len, writer)?;

        self.inner.encode(writer)
    }
}

impl MetaInfo for ExplicitTag<'_> {
    fn clear_meta(&mut self) {
        self.inner.clear_meta()
    }
}
