use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

/// [BmpString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bmpstring.html)
///
/// The ASN.1 BMPString type contains UNICODE characters. They are two-byte characters, and are not recommended for use unless properly subtyped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BmpString<'data>(Cow<'data, [u8]>);

pub type OwnedBmpString = BmpString<'static>;

impl BmpString<'_> {
    pub const TAG: Tag = Tag(30);

    /// Returns inner raw [BmpString] data
    pub fn raw_data(&self) -> &[u8] {
        &self.0
    }

    /// Returns owned version of the [BmpString]
    pub fn to_owned(&self) -> OwnedBmpString {
        BmpString(self.0.to_vec().into())
    }

    pub fn new_owned(data: Vec<u8>) -> OwnedBmpString {
        BmpString(Cow::Owned(data))
    }
}

impl From<&str> for OwnedBmpString {
    fn from(value: &str) -> Self {
        Self(Cow::Owned(value.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()))
    }
}

impl Taggable for BmpString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for BmpString<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.remaining();

        if data.len() % 2 == 1 {
            return Err("Invalid BmpString".into());
        }

        Ok(Self(Cow::Borrowed(data)))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for BmpString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.len();
        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.0.len(), writer)?;
        writer.write_slice(&self.0)
    }
}
