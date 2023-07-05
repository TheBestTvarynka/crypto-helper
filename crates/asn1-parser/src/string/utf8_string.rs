use alloc::borrow::Cow;
use alloc::string::String;
use core::str::from_utf8;

use crate::length::read_len;
use crate::reader::{read_data, Reader};
use crate::{Asn1, Asn1Decode, Asn1Entity, Asn1Result, Asn1Type, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8String<'data> {
    string: Cow<'data, str>,
}

pub type OwnedUtf8String = Utf8String<'static>;

impl Utf8String<'_> {
    pub const TAG: Tag = Tag(12);
}

impl From<String> for OwnedUtf8String {
    fn from(data: String) -> Self {
        Self {
            string: Cow::Owned(data),
        }
    }
}

impl<'data> Asn1Decode<'data> for Utf8String<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        Utf8String::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let (data, _data_range) = read_data(reader, len)?;

        Ok(Self {
            string: Cow::Borrowed(from_utf8(data)?),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: reader.data_in_range(tag_position..data_range.end)?,
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Asn1Type::Utf8String(Self {
                string: Cow::Borrowed(from_utf8(data)?),
            }),
        })
    }
}

impl Asn1Entity for Utf8String<'_> {
    fn tag(&self) -> &Tag {
        &Utf8String::TAG
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use crate::reader::Reader;
    use crate::{Asn1Decode, Asn1Type, Utf8String};

    #[test]
    fn decoding_example() {
        let raw = [
            12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97,
        ];

        let utf8_string = Utf8String::decode_asn1(&mut Reader::new(&raw)).unwrap();

        assert_eq!(utf8_string.tag_position(), 0);
        assert_eq!(utf8_string.length_bytes(), &[15]);
        assert_eq!(utf8_string.length_range(), 1..2);
        assert_eq!(&raw[utf8_string.data_range()], b"thebesttvarynka");
        assert_eq!(
            utf8_string.asn1(),
            &Asn1Type::Utf8String(Utf8String {
                string: Cow::Borrowed("thebesttvarynka"),
            })
        );
    }
}
