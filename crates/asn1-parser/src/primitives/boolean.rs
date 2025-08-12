use crate::length::write_len;
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Error, Tag, Taggable};

/// [Boolen](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/boolean.html)
///
/// The ASN.1 BOOLEAN type has two possible values: TRUE and FALSE.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bool(bool);

impl Bool {
    pub const TAG: Tag = Tag(1);

    pub fn value(&self) -> bool {
        self.0
    }

    pub fn from_byte(byte: u8) -> Self {
        Bool(byte != 0)
    }
}

impl From<bool> for Bool {
    fn from(flag: bool) -> Self {
        Self(flag)
    }
}

impl Taggable for Bool {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl<'data> Asn1ValueDecoder<'data> for Bool {
    fn decode(_: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        let data = reader.remaining();

        if data.is_empty() {
            warn!("Bool data length is 0. Processing with the default value: `true`");

            Ok(Bool::from_byte(1))
        } else if data.len() == 1 {
            Ok(Bool::from_byte(data[0]))
        } else {
            Err(Error::from("Bool data len should be equal to 1"))
        }
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for Bool {
    fn needed_buf_size(&self) -> usize {
        1 /* tag */ + 1 /* len */ + 1 /* bool value */
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(1, writer)?;
        writer.write_byte(match self.0 {
            true => 0xff,
            false => 0,
        })
    }
}
