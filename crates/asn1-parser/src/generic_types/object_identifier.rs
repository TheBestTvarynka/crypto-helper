use alloc::vec::Vec;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, Tag, Taggable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectIdentifier(oid::ObjectIdentifier);

impl ObjectIdentifier {
    pub const TAG: Tag = Tag(0x06);

    pub fn oid(&self) -> &oid::ObjectIdentifier {
        &self.0
    }
}

impl From<oid::ObjectIdentifier> for ObjectIdentifier {
    fn from(value: oid::ObjectIdentifier) -> Self {
        Self(value)
    }
}

impl Taggable for ObjectIdentifier {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1ValueDecoder<'_> for ObjectIdentifier {
    fn decode(_tag: Tag, reader: &mut Reader<'_>) -> Asn1Result<Self> {
        Ok(Self(oid::ObjectIdentifier::try_from(reader.remaining())?))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl Asn1Encoder for ObjectIdentifier {
    fn needed_buf_size(&self) -> usize {
        let encoded: Vec<u8> = self.0.clone().into();
        encoded.len()
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        let encoded: Vec<u8> = self.0.clone().into();
        writer.write_slice(&encoded)
    }
}
