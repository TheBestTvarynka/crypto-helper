#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

mod asn1;
mod constructors;
mod error;
mod length;
mod mutable;
mod primitives;
mod reader;
mod string;
mod tag;
mod tags;
mod time;
mod tlv;
mod writer;

pub use asn1::{Asn1, Asn1Type, RawAsn1EntityData};
pub use constructors::*;
pub use error::Error;
pub use mutable::{IntoMutable, Mutable};
pub use primitives::*;
use reader::Reader;
pub use string::*;
pub use tag::Tag;
pub use tags::*;
pub use time::*;
pub use tlv::Tlv;
use writer::Writer;

pub type Asn1Result<T> = Result<T, Error>;

/// General trait for decoding asn1 entities.
pub trait Asn1Decoder<'data>: Sized {
    /// Check if the provided tag belongs to decoding implementation.
    fn compare_tags(tag: Tag) -> bool;

    /// Decodes the asn1 entity using provided Reader.
    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self>;

    /// Decodes the asn1 entity using provided buffer.
    fn decode_buff(buff: &'data [u8]) -> Asn1Result<Self> {
        Self::decode(&mut Reader::new(buff))
    }
}

/// Decodes the provided data into the vector of asn1 trees.
pub fn decode_buff_vec(buff: &[u8]) -> Asn1Result<Vec<Asn1>> {
    let mut reader = Reader::new(buff);

    let mut trees = Vec::new();

    while !reader.empty() {
        trees.push(Asn1::decode(&mut reader)?);
    }

    Ok(trees)
}

pub trait Asn1ValueDecoder<'data>: Sized {
    fn decode(tag: Tag, reader: &mut Reader<'data>) -> Asn1Result<Self>;

    fn compare_tags(tag: Tag) -> bool;
}

/// General trait for encoding asn1 entities
pub trait Asn1Encoder {
    /// Returns needed buffer size for asn1 entity encoding
    fn needed_buf_size(&self) -> usize;

    /// Encodes asn1 entity into provided buffer
    fn encode_buff(&self, buf: &mut [u8]) -> Asn1Result<()> {
        self.encode(&mut Writer::new(buf))
    }

    /// Encodes asn1 entity into provided writer
    fn encode(&self, writer: &mut Writer) -> Asn1Result<()>;
}

impl Asn1Encoder for &[Asn1] {
    fn needed_buf_size(&self) -> usize {
        self.iter().map(|tree| tree.needed_buf_size()).sum()
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        for tree in self.iter() {
            tree.encode(writer)?;
        }

        Ok(())
    }
}

/// Every asn1 entity should implement this trait.
pub trait Asn1Entity {
    /// Returns asn1 tag of the entity
    fn tag(&self) -> Tag;

    /// Returns a unique asn1 node id
    fn id(&self) -> u64;
}

pub trait Taggable {
    /// Returns asn1 tag of the entity
    fn tag(&self) -> Tag;
}

pub trait MetaInfo {
    fn clear_meta(&mut self);
}
