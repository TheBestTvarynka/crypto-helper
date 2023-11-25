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

        let mut decoded = Asn1Type::decode_buff(&buff).unwrap();

        assert_eq!(decoded.needed_buf_size(), buff_len);
        assert_eq!(decoded.tag(), asn1_tag);

        decoded.clear_raw_data();
        assert_eq!(decoded, asn1);
    })
}

#[test]
fn asn1() {
    proptest!(|(asn1 in any_asn1_type())| {
        let asn1_tag = asn1.tag();

        let buff_len = asn1.needed_buf_size();
        let mut buff = vec![0; buff_len];

        asn1.encode_buff(&mut buff).unwrap();

        let mut decoded = Asn1Type::decode_asn1_buff(&buff).unwrap();

        assert_eq!(decoded.asn1().needed_buf_size(), buff_len);
        assert_eq!(1 + decoded.raw_entity_data().length_bytes().len() + decoded.raw_entity_data().data_bytes().len(), buff_len);
        assert_eq!(decoded.asn1().tag(), asn1_tag);
        assert_eq!(decoded.raw_entity_data().tag_position(), 0);
        assert_eq!(decoded.raw_entity_data().raw_bytes(), buff);

        decoded.clear_raw_data();
        assert_eq!(decoded.asn1(), &asn1);
    })
}

#[test]
fn full_example() {
    use asn1_parser::*;

    let asn1 = Sequence::new(vec![
        Asn1::new(Default::default(), Box::new(Asn1Type::Bool(true.into()))),
        Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(0, Asn1::new(Default::default(), Box::new(Asn1Type::Utf8String("TbeBestTvarynka".into()))))))),
        Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(1, Asn1::new(Default::default(), Box::new(Asn1Type::Sequence(Sequence::new(vec![
            Asn1::new(Default::default(), Box::new(Asn1Type::Null(Null))),
            Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(4, Asn1::new(Default::default(), Box::new(Asn1Type::OctetString(vec![48, 5, 160, 3, 1, 1, 255].into()))))))),
            Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(4, Asn1::new(Default::default(), Box::new(Asn1Type::BitString(BitString::from_raw_vec(32, vec![64, 129, 0, 16]).unwrap()))))))),
            Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(4, Asn1::new(Default::default(), Box::new(Asn1Type::ApplicationTag(ApplicationTag::new(12, Asn1::new(Default::default(), Box::new(Asn1Type::Sequence(Sequence::new(vec![
                Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(0, Asn1::new(Default::default(), Box::new(Asn1Type::Null(Null))))))),
                Asn1::new(Default::default(), Box::new(Asn1Type::ExplicitTag(ExplicitTag::new(1, Asn1::new(Default::default(), Box::new(Asn1Type::BmpString("Certificate".into()))))))),
            ])))))))))))),
        ])))))))),
    ]);

    let buff_len = asn1.needed_buf_size();
    let mut buff = vec![0; buff_len];

    asn1.encode_buff(&mut buff).unwrap();

    println!("{:?}", buff);
}
