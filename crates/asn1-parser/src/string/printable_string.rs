use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::str::from_utf8;

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

/// [PrintableString](https://obj-sys.com/asn1tutorial/node128.html)
///
/// a-z, A-Z, ' () +,-.?:/= and SPACE
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintableString<'data>(Cow<'data, str>);

pub type OwnedPrintableString = PrintableString<'static>;

impl PrintableString<'_> {
    pub const TAG: Tag = Tag(19);

    /// Returns inner raw data
    pub fn raw_data(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns inner string data
    pub fn string(&self) -> &str {
        &self.0
    }

    /// Returns owned version of the [PrintableString]
    pub fn to_owned(&self) -> OwnedPrintableString {
        PrintableString(self.0.to_string().into())
    }

    fn validate(data: &str) -> bool {
        const ALLOWED_SPECIAL: &[u8] = &[b' ', b'\'', b'(', b')', b'+', b',', b'-', b'.', b'/', b':', b'=', b'?'];

        for c in data.as_bytes() {
            if !(c.is_ascii_lowercase()
                || c.is_ascii_uppercase()
                || c.is_ascii_digit()
                || ALLOWED_SPECIAL.contains(c))
            {
                return false;
            }
        }

        true
    }

    pub fn new_owned(string: String) -> Asn1Result<OwnedPrintableString> {
        if Self::validate(&string) {
            return Err("invalid printable string data".into());
        }
        Ok(PrintableString(Cow::Owned(string)))
    }
}

impl<'data> Asn1ValueDecoder<'data> for PrintableString<'data> {
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

impl Taggable for PrintableString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for PrintableString<'_> {
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
