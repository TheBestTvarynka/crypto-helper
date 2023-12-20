use core::ops::Range;

use crate::{Asn1Result, Error};

#[derive(Debug)]
pub struct Reader<'data> {
    next_node_id: u64,
    offset: usize,
    position: usize,
    inner: &'data [u8],
}

impl<'data> Reader<'data> {
    pub fn new(data: &'data [u8]) -> Self {
        Self {
            offset: 0,
            next_node_id: 0,
            position: 0,
            inner: data,
        }
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn next_id(&mut self) -> u64 {
        self.next_node_id += 1;
        self.next_node_id
    }

    pub fn set_next_id(&mut self, next_node_id: u64) {
        self.next_node_id = next_node_id;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn full_offset(&self) -> usize {
        self.offset + self.position
    }

    pub fn empty(&self) -> bool {
        self.position == self.inner.len()
    }

    pub fn remaining(&self) -> &'data [u8] {
        &self.inner[self.position..]
    }

    pub fn read_remaining(&mut self) -> &'data [u8] {
        let data = &self.inner[self.position..];
        self.position = self.inner.len();
        data
    }

    pub fn data_in_range(&self, range: Range<usize>) -> Asn1Result<&'data [u8]> {
        if range.end > self.inner.len() {
            return Err(Error::from("Invalid range"));
        }

        Ok(&self.inner[range])
    }

    pub fn read(&mut self, len: usize) -> Asn1Result<&'data [u8]> {
        if self.position + len > self.inner.len() {
            return Err(Error::from("Outside"));
        }

        let data = &self.inner[self.position..(self.position + len)];

        self.position += len;

        Ok(data)
    }

    pub fn read_exact(&mut self, buff: &mut [u8]) -> Asn1Result<()> {
        buff.copy_from_slice(self.read(buff.len())?);

        Ok(())
    }

    pub fn read_byte(&mut self) -> Asn1Result<u8> {
        Ok(self.read(1)?[0])
    }

    pub fn peek_byte(&self) -> Asn1Result<u8> {
        if self.position == self.inner.len() {
            return Err(Error::from("End of the buffer"));
        }

        Ok(self.inner[self.position])
    }
}

pub fn read_data<'data>(reader: &mut Reader<'data>, len: usize) -> Asn1Result<(&'data [u8], Range<usize>)> {
    let before = reader.position();

    let data = reader.read(len)?;

    let after = reader.position();

    Ok((data, before..after))
}
