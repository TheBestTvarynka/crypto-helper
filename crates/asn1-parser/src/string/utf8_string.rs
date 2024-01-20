use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::str::from_utf8;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

/// [Utf8String](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/utf8string.html)
///
/// The ASN.1 UTF8String type is used for handling Unicode characters. UniversalString and UTF8String both support the same character set,
/// however, their encoding is different.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8String<'data>(Cow<'data, str>);

pub type OwnedUtf8String = Utf8String<'static>;

impl Utf8String<'_> {
    pub const TAG: Tag = Tag(12);

    /// Returns inner raw data
    pub fn raw_data(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns inner string data
    pub fn string(&self) -> &str {
        &self.0
    }

    /// Returns owned version of the [Utf8String]
    pub fn to_owned(&self) -> OwnedUtf8String {
        Utf8String(self.0.to_string().into())
    }

    pub fn new_owned(string: String) -> OwnedUtf8String {
        Utf8String(Cow::Owned(string))
    }
}

impl From<String> for OwnedUtf8String {
    fn from(data: String) -> Self {
        Self(Cow::Owned(data))
    }
}

impl From<&'static str> for OwnedUtf8String {
    fn from(data: &'static str) -> Self {
        Self(Cow::Borrowed(data))
    }
}

impl<'data> Asn1ValueDecoder<'data> for Utf8String<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        Ok(Self(Cow::Borrowed(from_utf8(reader.remaining())?)))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Taggable for Utf8String<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for Utf8String<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.0.len(), writer)?;
        writer.write_slice(self.0.as_bytes())
    }
}
