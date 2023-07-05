use asn1_parser::{OwnedOctetString, OwnedUtf8String};
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
