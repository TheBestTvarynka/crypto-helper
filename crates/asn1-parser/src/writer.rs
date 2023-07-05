use crate::{Asn1Result, Error};

#[derive(Debug)]
pub struct Writer<'data> {
    position: usize,
    inner: &'data mut [u8],
}

impl<'data> Writer<'data> {
    pub fn new(data: &'data mut [u8]) -> Self {
        Self {
            position: 0,
            inner: data,
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Asn1Result<()> {
        if self.position == self.inner.len() {
            return Err(Error::from("Buffer is too small"));
        }

        self.inner[self.position] = byte;
        self.position += 1;

        Ok(())
    }

    pub fn write_slice(&mut self, slice: &[u8]) -> Asn1Result<()> {
        let slice_len = slice.len();
        if self.position + slice_len > self.inner.len() {
            return Err(Error::from("Buffer is too small"));
        }

        self.inner[self.position..self.position + slice_len].copy_from_slice(slice);
        self.position += slice_len;

        Ok(())
    }
}
