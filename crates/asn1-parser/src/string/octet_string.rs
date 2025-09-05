use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1ValueDecoder, IntoMutable, MetaInfo, Mutable, Tag, Taggable};

/// [OctetString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/octetstring.html)
///
/// The ASN.1 OCTET STRING type contains arbitrary strings of octets. This type is very similar to BIT STRING,
/// except that all values must be an integral number of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    octets: Cow<'data, [u8]>,
    inner: Option<Box<Asn1<'data>>>,
}

pub type OwnedOctetString = OctetString<'static>;

impl OctetString<'_> {
    pub const TAG: Tag = Tag(4);

    /// Returns inner octets
    pub fn octets(&self) -> &[u8] {
        &self.octets
    }

    pub fn inner(&self) -> Option<&Asn1<'_>> {
        self.inner.as_ref().map(|i| i.as_ref())
    }

    /// Returns owned version of the [OctetString]
    pub fn to_owned(&self) -> OwnedOctetString {
        OctetString {
            octets: self.octets.to_vec().into(),
            inner: self
                .inner
                .as_ref()
                .map(|inner| Box::new(inner.to_owned_with_asn1(inner.inner_asn1().to_owned()))),
        }
    }

    pub fn new_owned(octets: Vec<u8>) -> OwnedOctetString {
        let inner = Asn1::decode_buff(&octets).ok().map(|mut asn1| {
            asn1.clear_meta();
            Box::new(asn1.to_owned_with_asn1(asn1.inner_asn1().to_owned()))
        });

        OwnedOctetString {
            octets: Cow::Owned(octets),
            inner,
        }
    }
}

impl From<Vec<u8>> for OwnedOctetString {
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

impl<'data> Asn1ValueDecoder<'data> for OctetString<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.read_remaining();

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset() - data.len());
        let mut inner = Asn1::decode(&mut inner_reader).ok().map(Box::new);

        if !inner_reader.empty() && inner.is_some() {
            inner = None;
        }

        reader.set_next_id(inner_reader.next_id());

        Ok(Self {
            octets: Cow::Borrowed(data),
            inner,
        })
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl IntoMutable<OwnedOctetString> for OctetString<'_> {
    fn into_mutable(self) -> Mutable<OwnedOctetString> {
        let OctetString { octets, inner: _ } = self;
        Mutable::new(OctetString {
            octets: match octets {
                Cow::Owned(v) => Cow::Owned(v),
                Cow::Borrowed(v) => Cow::Owned(v.to_vec()),
            },
            inner: None,
        })
    }
}

impl Taggable for OctetString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for OctetString<'_> {
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

impl MetaInfo for OctetString<'_> {
    fn clear_meta(&mut self) {
        self.inner = None;
    }
}
