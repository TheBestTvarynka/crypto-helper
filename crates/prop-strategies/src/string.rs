use asn1_parser::{BitString, OwnedBitString, OwnedBmpString, OwnedOctetString, OwnedUtf8String};
use proptest::prop_compose;

use crate::{bytes, string};

const STRING_LEN: usize = 12;

prop_compose! {
    pub fn any_octet_string()
        (data in bytes(STRING_LEN)) -> OwnedOctetString {
        OwnedOctetString::new_owned(0, data)
    }
}

prop_compose! {
    pub fn any_utf8_string()
        (data in string(STRING_LEN)) -> OwnedUtf8String {
        OwnedUtf8String::new_owned(0, data)
    }
}

prop_compose! {
    pub fn any_bit_string()
        (
            data in bytes(STRING_LEN),
            unused_bits in 0..8_usize,
        ) -> OwnedBitString {
        BitString::from_raw_vec(
            0,
            if data.is_empty() { 0 } else { data.len() * 8 - unused_bits },
            data,
        ).unwrap()
    }
}

prop_compose! {
    pub fn any_bmp_string()
        (data in string(STRING_LEN)) -> OwnedBmpString {
            OwnedBmpString::new_owned(0, data.into())
        }
}
