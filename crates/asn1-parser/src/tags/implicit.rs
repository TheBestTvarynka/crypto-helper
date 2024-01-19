use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitTag<'data> {
    tag: u8,
    octets: Cow<'data, [u8]>,
    inner: Option<Box<Asn1<'data>>>,
}

pub type OwnedImplicitTag = ImplicitTag<'static>;

impl<'data> ImplicitTag<'data> {
    pub fn new_owned(tag: u8, octets: Vec<u8>) -> Self {
        let inner = Asn1::decode_buff(&octets).ok().map(|mut asn1| {
            asn1.clear_meta();
            Box::new(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned()))
        });

        Self {
            tag,
            octets: Cow::Owned(octets),
            inner,
        }
    }

    pub fn tag_number(&self) -> u8 {
        self.tag & 0x1f
    }

    pub fn inner_asn1(&self) -> Option<&Asn1<'data>> {
        self.inner.as_ref().map(|asn1| asn1.as_ref())
    }

    pub fn to_owned(&self) -> OwnedImplicitTag {
        OwnedImplicitTag {
            tag: self.tag,
            octets: self.octets.to_vec().into(),
            inner: self
                .inner
                .as_ref()
                .map(|inner| Box::new(inner.to_owned_with_asn1(inner.inner_asn1().to_owned()))),
        }
    }
}

impl Taggable for ImplicitTag<'_> {
    fn tag(&self) -> Tag {
        Tag(self.tag)
    }
}

impl<'data> Asn1ValueDecoder<'data> for ImplicitTag<'data> {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.read_remaining();

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset() - data.len());
        let inner = Asn1::decode(&mut inner_reader).ok().map(Box::new);

        if !inner_reader.empty() && inner.is_some() {
            return Err("implicit tag inner data contains leftovers".into());
        }

        reader.set_next_id(inner_reader.next_id());

        Ok(Self {
            tag: tag.0,
            octets: Cow::Borrowed(data),
            inner,
        })
    }

    fn compare_tags(tag: Tag) -> bool {
        tag.is_context_specific() && tag.is_primitive()
    }
}

impl Asn1Encoder for ImplicitTag<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.octets.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(self.tag)?;
        write_len(self.octets.len(), writer)?;
        writer.write_slice(&self.octets)
    }
}

impl MetaInfo for ImplicitTag<'_> {
    fn clear_meta(&mut self) {
        if let Some(asn1) = self.inner.as_mut() {
            asn1.clear_meta()
        }
    }
}
