use asn1_parser::{Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Type};
use prop_strategies::any_asn1_type;
use proptest::proptest;

#[test]
fn asn1_type() {
    proptest!(|(asn1 in any_asn1_type())| {
        let asn1_tag = asn1.tag();

        let buff_len = asn1.needed_buf_size();
        let mut buff = vec![0; buff_len];

        asn1.encode_buff(&mut buff).unwrap();

        let decoded = Asn1Type::decode_buff(&buff).unwrap();

        assert_eq!(decoded.needed_buf_size(), buff_len);
        assert_eq!(decoded, asn1);
        assert_eq!(decoded.tag(), asn1_tag);
    })
}

#[test]
fn asn1() {
    proptest!(|(asn1 in any_asn1_type())| {
        let asn1_tag = asn1.tag();

        let buff_len = asn1.needed_buf_size();
        let mut buff = vec![0; buff_len];

        asn1.encode_buff(&mut buff).unwrap();

        let decoded = Asn1Type::decode_asn1_buff(&buff).unwrap();

        assert_eq!(decoded.asn1().needed_buf_size(), buff_len);
        assert_eq!(1 + decoded.raw_entity_data().length_bytes().len() + decoded.raw_entity_data().data_bytes().len(), buff_len);
        assert_eq!(decoded.asn1(), &asn1);
        assert_eq!(decoded.asn1().tag(), asn1_tag);
        assert_eq!(decoded.raw_entity_data().tag_position(), 0);
        assert_eq!(decoded.raw_entity_data().raw_bytes(), buff);
    })
}
