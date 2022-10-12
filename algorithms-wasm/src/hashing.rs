use sha1::{Sha1, Digest};

pub fn sha1(payload: &[u8]) -> Vec<u8> {
    let mut sha1 = Sha1::new();
    sha1.update(payload);
    
    sha1.finalize().to_vec()
}
