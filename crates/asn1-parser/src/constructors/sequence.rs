use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Tag};

/// [ASN.1 SEQUENCE](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/sequence.html)
///
/// In ASN.1, an ordered list of elements (or components) comprises a SEQUENCE.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sequence<'data> {
    fields: Vec<Asn1<'data>>,
}

pub type OwnedSequence = Sequence<'static>;

impl Sequence<'_> {
    pub const TAG: Tag = Tag(0x30);

    pub fn new(fields: Vec<Asn1>) -> Sequence {
        Sequence { fields }
    }

    /// Retuens [Sequence] fields
    pub fn fields(&self) -> &[Asn1<'_>] {
        &self.fields
    }

    /// Returns owned version of the [Sequence]
    pub fn to_owned(&self) -> OwnedSequence {
        Sequence {
            fields: self.fields.iter().map(|f| f.to_owned()).collect(),
        }
    }

    pub fn clear_raw_data(&mut self) -> &mut Self {
        self.fields.iter_mut().for_each(|asn1| {
            asn1.clear_raw_data();
        });
        self
    }
}

impl<'data> From<Vec<Asn1<'data>>> for Sequence<'data> {
    fn from(fields: Vec<Asn1<'data>>) -> Self {
        Self { fields }
    }
}

impl Asn1Entity for Sequence<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for Sequence<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.fields.iter().map(|f| f.asn1().needed_buf_size()).sum();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;

        let data_len = self.fields.iter().map(|f| f.asn1().needed_buf_size()).sum();
        write_len(data_len, writer)?;

        self.fields.iter().try_for_each(|f| f.asn1().encode(writer))
    }
}

impl<'data> Asn1Decoder<'data> for Sequence<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        &Self::TAG == tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let mut fields = Vec::new();

        let position = reader.position();
        while reader.position() - position < len {
            fields.push(Asn1Type::decode_asn1(reader)?);
        }

        Ok(Self { fields })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;
        let data_range = len_range.end..len_range.end + len;

        let mut fields = Vec::new();

        let position = reader.position();
        while reader.position() - position < len {
            fields.push(Asn1Type::decode_asn1(reader)?);
        }

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(tag_position..len_range.end + len)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::Sequence(Sequence { fields })),
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use alloc::vec;

    use crate::asn1::RawAsn1EntityData;
    use crate::{Asn1, Asn1Decoder, Asn1Type, OctetString, Sequence, Utf8String};

    #[test]
    fn example() {
        let raw = [
            48, 27, 4, 8, 0, 17, 34, 51, 68, 85, 102, 119, 12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114,
            121, 110, 107, 97,
        ];

        let decoded = Sequence::decode_asn1_buff(&raw).unwrap();

        assert_eq!(
            decoded,
            Asn1 {
                raw_data: RawAsn1EntityData {
                    raw_data: Cow::Borrowed(&raw),
                    tag: 0,
                    length: 1..2,
                    data: 2..29,
                },
                asn1_type: Box::new(Asn1Type::Sequence(Sequence {
                    fields: vec![
                        Asn1 {
                            raw_data: RawAsn1EntityData {
                                raw_data: Cow::Borrowed(&[4, 8, 0, 17, 34, 51, 68, 85, 102, 119]),
                                tag: 2,
                                length: 3..4,
                                data: 4..12,
                            },
                            asn1_type: Box::new(Asn1Type::OctetString(OctetString::from(vec![
                                0, 17, 34, 51, 68, 85, 102, 119
                            ]))),
                        },
                        Asn1 {
                            raw_data: RawAsn1EntityData {
                                raw_data: Cow::Borrowed(&[
                                    12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97
                                ]),
                                tag: 12,
                                length: 13..14,
                                data: 14..29,
                            },
                            asn1_type: Box::new(Asn1Type::Utf8String(Utf8String::from("thebesttvarynka")))
                        },
                    ]
                })),
            }
        );
    }
}
