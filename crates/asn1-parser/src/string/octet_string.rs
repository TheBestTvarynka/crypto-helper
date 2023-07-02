#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    octets: &'data [u8],
}
