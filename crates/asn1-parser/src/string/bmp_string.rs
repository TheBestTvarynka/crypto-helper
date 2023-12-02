use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Tag};

/// [BmpString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/bmpstring.html)
///
/// The ASN.1 BMPString type contains UNICODE characters. They are two-byte characters, and are not recommended for use unless properly subtyped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BmpString<'data> {
    id: u64,
    data: Cow<'data, [u8]>,
}

pub type OwnedBmpString = BmpString<'static>;

impl BmpString<'_> {
    pub const TAG: Tag = Tag(30);

    /// Returns inner raw [BmpString] data
    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }

    /// Returns owned version of the [BmpString]
    pub fn to_owned(&self) -> OwnedBmpString {
        BmpString {
            id: self.id,
            data: self.data.to_vec().into(),
        }
    }
}

// impl From<&str> for OwnedBmpString {
//     fn from(value: &str) -> Self {
//         let data: Vec<u8> = value.encode_utf16().flat_map(|c| c.to_be_bytes()).collect();
//         Self { data: Cow::Owned(data) }
//     }
// }

impl Asn1Entity for BmpString<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<'data> Asn1Decoder<'data> for BmpString<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        Self::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        if len % 2 == 1 {
            return Err("Invalid BmpString".into());
        }

        let data = reader.read(len)?;

        Ok(Self {
            id: reader.next_id(),
            data: Cow::Borrowed(data),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(tag_position..data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::BmpString(Self {
                id: reader.next_id(),
                data: Cow::Borrowed(data),
            })),
        })
    }
}

impl Asn1Encoder for BmpString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.data.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.data.len(), writer)?;
        writer.write_slice(&self.data)
    }
}
