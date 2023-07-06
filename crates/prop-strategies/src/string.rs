use asn1_parser::{BitString, OwnedBitString, OwnedOctetString, OwnedUtf8String};
use proptest::prop_compose;

use crate::{bytes, string};

prop_compose! {
    pub fn any_octet_string()
        (data in bytes(1024)) -> OwnedOctetString {
        OwnedOctetString::from(data)
    }
}

prop_compose! {
    pub fn any_utf8_string()
        (data in string(1024)) -> OwnedUtf8String {
        OwnedUtf8String::from(data)
    }
}

prop_compose! {
    pub fn any_bit_string()
        (
            data in bytes(1024),
            unused_bits in 0..8_usize,
        ) -> OwnedBitString {
        BitString::from_raw_vec(
            if data.is_empty() { 0 } else { data.len() * 8 - unused_bits },
            data,
        ).unwrap()
    }
}
