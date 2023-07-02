#![no_std]

extern crate alloc;

mod asn1;
mod constructors;
mod error;
mod length;
mod reader;
mod string;
mod tag;

pub use asn1::{Asn1, Asn1Type};
pub use constructors::*;
pub use error::Error;
use reader::Reader;
pub use string::*;
pub use tag::Tag;

pub type Asn1Result<T> = Result<T, Error>;

/// General trait for decoding asn1 entities.
pub trait Asn1Decode: Sized {
    /// Check if the provided tag belongs to decoding implementation.
    fn compare_tags(tag: &Tag) -> bool;

    /// Decodes the asn1 entity using provided Reader.
    fn decode(reader: &mut Reader) -> Asn1Result<Self>;
}

/// Every asn1 entity should implement this trait.
pub trait Asn1Entity {
    /// Returns asn1 tag of the entity
    fn tag(&self) -> &Tag;
}
