use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Decoder, Asn1Encoder, Asn1Result, Asn1Type, Asn1ValueDecoder, Tag, Taggable};

/// [OctetString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/octetstring.html)
///
/// The ASN.1 OCTET STRING type contains arbitrary strings of octets. This type is very similar to BIT STRING,
/// except that all values must be an integral number of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    octets: Cow<'data, [u8]>,
    inner: Option<Box<Asn1Type<'data>>>,
}

pub type OwnedOctetString = OctetString<'static>;

impl OctetString<'_> {
    pub const TAG: Tag = Tag(4);

    /// Returns inner octets
    pub fn octets(&self) -> &[u8] {
        &self.octets
    }

    pub fn inner(&self) -> Option<&Asn1Type<'_>> {
        self.inner.as_ref().map(|i| i.as_ref())
    }

    /// Returns owned version of the [OctetString]
    pub fn to_owned(&self) -> OwnedOctetString {
        OctetString {
            octets: self.octets.to_vec().into(),
            inner: self.inner.as_ref().map(|inner| Box::new(inner.to_owned())),
        }
    }

    pub fn new_owned(octets: Vec<u8>) -> OwnedOctetString {
        let inner = Asn1Type::decode_buff(&octets)
            .ok()
            .map(|asn1| Box::new(asn1.to_owned()));
        OwnedOctetString {
            octets: Cow::Owned(octets),
            inner,
        }
    }
}

impl From<Vec<u8>> for OwnedOctetString {
    fn from(data: Vec<u8>) -> Self {
        let inner = Asn1Type::decode_buff(&data).ok().map(|asn1| Box::new(asn1.to_owned()));
        Self {
            octets: Cow::Owned(data),
            inner,
        }
    }
}

impl<'data> Asn1ValueDecoder<'data> for OctetString<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.remaining();

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset() - data.len());
        let inner = Asn1Type::decode(&mut inner_reader).ok().map(Box::new);

        if !inner_reader.empty() {
            return Err("octet string inner data contains leftovers".into());
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

impl Taggable for OctetString<'_> {
    fn tag(&self) -> Tag {
        OctetString::TAG
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
