use alloc::borrow::Cow;
use alloc::boxed::Box;

use crate::length::{read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Error, RawAsn1EntityData, Tag};

/// [Null](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/null.html)
///
/// The ASN.1 NULL type is used when you need a placeholder for which there is no value.
/// For example, it can be used to mark a currently empty space.
/// The NULL type has only one possible value, also called NULL.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Null;

impl Null {
    pub const TAG: Tag = Tag(5);
}

impl Asn1Entity for Null {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1Decoder<'data> for Null {
    fn compare_tags(tag: &Tag) -> bool {
        *tag == Self::TAG
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        if len != 0 {
            return Err(Error::from("Bool length must be equal to 0"));
        }

        Ok(Self)
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        if len != 0 {
            return Err(Error::from("Bool length must be equal to 0"));
        }

        let (_, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(tag_position..data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::Null(Self)),
        })
    }
}

impl Asn1Encoder for Null {
    fn needed_buf_size(&self) -> usize {
        1 /* tag */ + 1 /* length (always zero) */
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(0, writer)?;

        Ok(())
    }
}
