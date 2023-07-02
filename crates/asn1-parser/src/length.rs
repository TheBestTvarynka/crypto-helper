use core::mem::size_of;
use core::ops::Range;

use crate::reader::Reader;
use crate::{Asn1Result, Error};

const USIZE_LEN: usize = size_of::<usize>();

pub fn read_len(reader: &mut Reader) -> Asn1Result<(usize, Range<usize>)> {
    let before = reader.position();

    let length = match reader.read_byte()? {
        n @ 128..=255 => {
            let len = n as usize & 127;
            if len > USIZE_LEN {
                return Err(Error::from("Invalid length bytes"));
            }

            let mut num = [0; USIZE_LEN];
            reader.read_exact(&mut num[USIZE_LEN - len..])?;

            usize::from_be_bytes(num)
        }
        n => n as usize,
    };

    let after = reader.position();

    Ok((length, before..after))
}
