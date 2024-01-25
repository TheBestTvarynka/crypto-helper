use alloc::vec::Vec;

use crate::asn1::Asn1;
use crate::length::write_len;
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Encoder, Asn1Result, Asn1ValueDecoder, MetaInfo, Sequence, Tag, Taggable};

/// [ASN.1 SET](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/set.html)
///
/// The ASN.1 SET type is similar to the SEQUENCE type. The key difference is that the elements
/// in each value of a SEQUENCE type must appear in the order shown in the definition.
/// The elements of a SET type value may appear in any order, regardless of how they are listed in the SET's definition
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Set<'data>(Sequence<'data>);

pub type OwnedSet = Set<'static>;

impl Set<'_> {
    /// Tag value of the [SET] type
    pub const TAG: Tag = Tag(0x31);

    /// Creates a new [Set] from passed fields
    pub fn new(fields: Vec<Asn1>) -> Set {
        Set(Sequence::new(fields))
    }

    /// Returns [Set] fields
    pub fn fields(&self) -> &[Asn1<'_>] {
        self.0.fields()
    }

    /// Returns owned version of the [Set]
    pub fn to_owned(&self) -> OwnedSet {
        Set(Sequence::from(
            self.0
                .fields()
                .iter()
                .map(|f| f.to_owned_with_asn1(f.inner_asn1().to_owned()))
                .collect::<Vec<_>>(),
        ))
    }
}

impl<'data> From<Vec<Asn1<'data>>> for Set<'data> {
    fn from(fields: Vec<Asn1<'data>>) -> Self {
        Self(Sequence::from(fields))
    }
}

impl Taggable for Set<'_> {
    fn tag(&self) -> Tag {
        Self::TAG
    }
}

impl Asn1Encoder for Set<'_> {
    fn needed_buf_size(&self) -> usize {
        self.0.needed_buf_size()
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;

        let data_len = self.0.fields().iter().map(|f| f.needed_buf_size()).sum();
        write_len(data_len, writer)?;

        self.0.fields().iter().try_for_each(|f| f.encode(writer))
    }
}

impl<'data> Asn1ValueDecoder<'data> for Set<'data> {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self> {
        Ok(Self(Sequence::decode(tag, reader)?))
    }

    fn compare_tags(tag: Tag) -> bool {
        Self::TAG == tag
    }
}

impl MetaInfo for Set<'_> {
    fn clear_meta(&mut self) {
        self.0.clear_meta()
    }
}
