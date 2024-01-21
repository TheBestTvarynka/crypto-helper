mod bit_string;
mod bmp_string;
mod octet_string;
mod validators;

use alloc::borrow::Cow;
use alloc::string::String;
use core::str::from_utf8;

pub use bit_string::{BitString, OwnedBitString};
pub use bmp_string::{BmpString, OwnedBmpString};
pub use octet_string::{OctetString, OwnedOctetString};
use validators::{validate_general, validate_ia5, validate_printable, validate_utf8};

use crate::length::{len_size, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Utf8Value<'data, const TAG: u8>(Cow<'data, str>);

type OwnedUtf8Value<const TAG: u8> = Utf8Value<'static, TAG>;

impl<const TAG: u8> Utf8Value<'_, TAG> {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<const TAG: u8> From<String> for OwnedUtf8Value<TAG> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'data, const TAG: u8> From<&'data str> for Utf8Value<'data, TAG> {
    fn from(value: &'data str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'data, const TAG: u8> Asn1ValueDecoder<'data> for Utf8Value<'data, TAG> {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        Ok(Self(Cow::Borrowed(from_utf8(reader.remaining())?)))
    }

    fn compare_tags(tag: Tag) -> bool {
        tag.0 == TAG
    }
}

impl<const TAG: u8> Asn1Encoder for Utf8Value<'_, TAG> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.0.len();
        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(TAG)?;
        write_len(self.0.len(), writer)?;
        writer.write_slice(self.0.as_bytes())
    }
}

impl_utf8_asn1!(PrintableString, 19, validate_printable);
impl_utf8_asn1!(Utf8String, 12, validate_utf8);
impl_utf8_asn1!(IA5String, 22, validate_ia5);
impl_utf8_asn1!(GeneralString, 27, validate_general);
