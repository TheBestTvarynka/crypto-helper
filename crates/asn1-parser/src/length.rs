use core::mem::size_of;
use core::ops::Range;

use crate::reader::Reader;
use crate::writer::Writer;
use crate::{Asn1Result, Error};

const USIZE_LEN: usize = size_of::<usize>();

/// Reads length from the reader
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

/// Writes asn1 length into provided writer
pub fn write_len(length: usize, writer: &mut Writer) -> Asn1Result<()> {
    if length < 128 {
        writer.write_byte(length.try_into()?)
    } else {
        let count_bytes: u8 = (USIZE_LEN - (length.leading_zeros() / 8) as usize).try_into()?;
        writer.write_byte(count_bytes | 0x80)?;

        let mut buf = [0; USIZE_LEN];
        buf.copy_from_slice(&length.to_be_bytes());
        writer.write_slice(&buf[USIZE_LEN - usize::from(count_bytes)..])
    }
}

/// Returns how many bytes encoded length will take
pub fn len_size(data_len: usize) -> usize {
    if data_len < 128 {
        1
    } else {
        1 + USIZE_LEN - (data_len.leading_zeros() / 8) as usize
    }
}
