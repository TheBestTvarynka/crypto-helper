use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::str::from_utf8;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

/// [IA5String](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/ia5string.html)
///
/// The ASN.1 IA5String type uses 7-bit characters. It is equivalent to the ASCII alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IA5String<'data>(Cow<'data, str>);

pub type OwnedIA5String = IA5String<'static>;

impl IA5String<'_> {
    pub const TAG: Tag = Tag(22);

    /// Returns inner raw data
    pub fn raw_data(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns inner string data
    pub fn string(&self) -> &str {
        &self.0
    }

    /// Returns owned version of the [IA5String]
    pub fn to_owned(&self) -> OwnedIA5String {
        IA5String(self.0.to_string().into())
    }

    fn validate(data: &str) -> bool {
        for c in data.chars() {
            if !c.is_ascii() {
                return false;
            }
        }
        true
    }

    pub fn new_owned(string: String) -> Asn1Result<OwnedIA5String> {
        if Self::validate(&string) {
            return Err("invalid ia5string data".into());
        }
        Ok(IA5String(Cow::Owned(string)))
    }
}

impl<'data> Asn1ValueDecoder<'data> for IA5String<'data> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = from_utf8(reader.remaining())?;
        if !Self::validate(data) {
            return Err("invalid ia5string data".into());
        }
        Ok(Self(Cow::Borrowed(data)))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Taggable for IA5String<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for IA5String<'_> {
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
