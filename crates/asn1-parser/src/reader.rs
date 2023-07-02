use crate::{Asn1Result, Error};

pub struct Reader<'data> {
    position: usize,
    inner: &'data [u8],
}

impl<'data> Reader<'data> {
    pub fn new(data: &'data [u8]) -> Self {
        Self {
            position: 0,
            inner: data,
        }
    }

    pub fn read(&mut self, len: usize) -> Asn1Result<&'data [u8]> {
        if self.position + len < self.inner.len() {
            return Err(Error::from("Outside"));
        }

        let data = &self.inner[self.position..(self.position + len)];

        self.position += len;

        Ok(data)
    }
}
