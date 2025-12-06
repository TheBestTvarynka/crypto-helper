use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{
    Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Mutable, Tag, Taggable, decode_buff_vec, decode_reader_vec,
};

/// [OctetString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/octetstring.html)
///
/// The ASN.1 OCTET STRING type contains arbitrary strings of octets. This type is very similar to BIT STRING,
/// except that all values must be an integral number of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctetString {
    octets: Vec<u8>,
    inner: Option<Mutable<Vec<Asn1>>>,
}

impl OctetString {
    pub const TAG: Tag = Tag(4);

    /// Returns inner octets
    pub fn octets(&self) -> &[u8] {
        &self.octets
    }

    pub fn inner(&self) -> Option<Mutable<Vec<Asn1>>> {
        self.inner.clone()
    }

    pub fn new(octets: Vec<u8>) -> OctetString {
        let inner = decode_buff_vec(&octets).ok().map(|mut asn1| {
            asn1.iter_mut().for_each(|tree| tree.clear_meta());
            Mutable::new(asn1)
        });

        OctetString { octets, inner }
    }

    pub fn set_octets(&mut self, octets: Vec<u8>) {
        self.octets = octets;
        self.inner = decode_buff_vec(&self.octets).ok().map(Mutable::new);
    }
}

impl From<Vec<u8>> for OctetString {
    fn from(data: Vec<u8>) -> Self {
        let inner = decode_buff_vec(&data).map(Mutable::new).ok();
        Self { octets: data, inner }
    }
}

impl<'data> Asn1ValueDecoder<'data> for OctetString {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.read_remaining();

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset() - data.len());
        let mut inner = decode_reader_vec(&mut inner_reader).ok().map(Mutable::new);

        if !inner_reader.empty() && inner.is_some() {
            inner = None;
        }

        reader.set_next_id(inner_reader.next_id());

        Ok(Self {
            octets: data.to_vec(),
            inner,
        })
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Taggable for OctetString {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for OctetString {
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

impl MetaInfo for OctetString {
    fn clear_meta(&mut self) {
        self.inner = None;
    }
}
