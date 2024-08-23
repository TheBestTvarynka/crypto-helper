use alloc::borrow::Cow;
use alloc::fmt::Debug;

use crate::length::read_len;
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{
    Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1ValueDecoder, MetaInfo, RawAsn1EntityData, Tag, Taggable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tlv<'data, A> {
    id: u64,
    meta: RawAsn1EntityData<'data>,
    asn1: A,
}

pub type OwnedTlv<A> = Tlv<'static, A>;

impl<A> Tlv<'_, A> {
    pub fn new(id: u64, raw: RawAsn1EntityData, asn1: A) -> Tlv<A> {
        Tlv { id, meta: raw, asn1 }
    }

    pub fn meta(&self) -> &RawAsn1EntityData {
        &self.meta
    }

    pub fn inner_asn1(&self) -> &A {
        &self.asn1
    }

    pub fn to_owned_with_asn1<B>(&self, asn1: B) -> OwnedTlv<B> {
        OwnedTlv {
            id: self.id,
            meta: self.meta.to_owned(),
            asn1,
        }
    }
}

impl<A: MetaInfo> MetaInfo for Tlv<'_, A> {
    fn clear_meta(&mut self) {
        self.id = Default::default();
        self.meta = Default::default();
        self.asn1.clear_meta()
    }
}

impl<A: Taggable> Asn1Entity for Tlv<'_, A> {
    fn tag(&self) -> Tag {
        self.asn1.tag()
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<'data, A: Asn1ValueDecoder<'data> + Debug> Asn1Decoder<'data> for Tlv<'data, A> {
    fn compare_tags(tag: Tag) -> bool {
        A::compare_tags(tag)
    }

    #[instrument(level = "debug", ret)]
    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let tag_position = reader.full_offset();
        let data_start = reader.position();

        let tag = Tag(reader.read_byte()?);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        trace!(?tag, ?len, ?data);

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset() - data.len());
        let asn1 = A::decode(tag, &mut inner_reader)?;

        reader.set_next_id(inner_reader.next_id());

        let raw_data = Cow::Borrowed(reader.data_in_range(data_start..data_range.end)?);
        let length = (len_range.start - data_start)..(len_range.end - data_start);
        let data = (data_range.start - data_start)..(data_range.end - data_start);

        Ok(Tlv {
            id: reader.next_id(),
            meta: RawAsn1EntityData {
                raw_data,
                tag: tag_position,
                length,
                data,
            },
            asn1,
        })
    }
}

impl<A: Asn1Encoder> Asn1Encoder for Tlv<'_, A> {
    fn needed_buf_size(&self) -> usize {
        self.asn1.needed_buf_size()
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        self.asn1.encode(writer)
    }
}
