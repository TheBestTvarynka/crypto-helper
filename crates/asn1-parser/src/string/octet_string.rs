use crate::length::read_len;
use crate::reader::{read_data, Reader};
use crate::{Asn1, Asn1Decode, Asn1Result, Asn1Type, Tag};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    octets: &'data [u8],
}

impl OctetString<'_> {
    pub const TAG: Tag = Tag(4);
}

impl<'data> Asn1Decode<'data> for OctetString<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        OctetString::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let (data, _data_range) = read_data(reader, len)?;

        Ok(Self { octets: data })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: reader.data_in_range(tag_position..data_range.end)?,
            tag: tag_position,
            length: len_range,
            data: data_range,
            asn1_type: Asn1Type::OctetString(Self { octets: data }),
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::println;

    use crate::reader::Reader;
    use crate::{Asn1Decode, OctetString};

    #[test]
    fn decoding_example() {
        let raw = [4, 8, 0, 17, 34, 51, 68, 85, 102, 119];

        let octet_string = OctetString::decode_asn1(&mut Reader::new(&raw)).unwrap();

        println!("{:?}", octet_string);

        assert_eq!(octet_string.tag_position(), 0);
        assert_eq!(octet_string.length_bytes(), &[8]);
        assert_eq!(octet_string.length_range(), 1..2);
        assert_eq!(&raw[octet_string.data_range()], &[0, 17, 34, 51, 68, 85, 102, 119])
    }
}
