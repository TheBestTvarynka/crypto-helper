use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Tag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BmpString<'data> {
    data: Cow<'data, [u16]>,
}

pub type OwnedBmpString = BmpString<'static>;

impl BmpString<'_> {
    pub const TAG: Tag = Tag(30);

    pub fn raw_data(&self) -> &[u16] {
        &self.data
    }

    pub fn to_owned(&self) -> OwnedBmpString {
        BmpString {
            data: self.data.to_vec().into(),
        }
    }
}
