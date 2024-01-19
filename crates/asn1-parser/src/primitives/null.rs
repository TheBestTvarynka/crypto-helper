use crate::length::write_len;
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Error, Tag, Taggable};

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

impl Taggable for Null {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for Null {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        if !reader.remaining().is_empty() {
            return Err(Error::from("Null data should be empty"));
        }

        Ok(Self)
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
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
