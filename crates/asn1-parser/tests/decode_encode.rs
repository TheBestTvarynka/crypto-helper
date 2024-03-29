use asn1_parser::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Type, MetaInfo, ObjectIdentifier, Taggable};
use prop_strategies::any_asn1_type;
use proptest::proptest;

#[test]
fn asn1() {
    proptest!(|(mut asn1 in any_asn1_type())| {
        let asn1_tag = asn1.tag();

        let buff_len = asn1.needed_buf_size();
        let mut buff = vec![0; buff_len];

        asn1.encode_buff(&mut buff).unwrap();

        let mut decoded = Asn1::decode_buff(&buff).unwrap();
        let decoded_inner_asn1 = decoded.inner_asn1();
        let decoded_meta = decoded.meta();

        assert_eq!(decoded_inner_asn1.needed_buf_size(), buff_len);
        assert_eq!(1 + decoded_meta.length_bytes().len() + decoded_meta.data_bytes().len(), buff_len);
        assert_eq!(decoded_inner_asn1.tag(), asn1_tag);
        assert_eq!(decoded_meta.tag_position(), 0);
        assert_eq!(decoded_meta.raw_bytes(), buff);

        decoded.clear_meta();
        asn1.clear_meta();
        assert_eq!(decoded.inner_asn1(), &asn1);
    })
}

#[test]
fn generalized_time() {
    let raw = [
        0x18, 0x0f, 0x32, 0x30, 0x33, 0x37, 0x30, 0x39, 0x31, 0x33, 0x30, 0x32, 0x34, 0x38, 0x30, 0x35, 0x5a,
    ];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);

    let raw = [24, 16, 49, 57, 56, 53, 49, 49, 48, 54, 50, 49, 48, 54, 50, 55, 46, 51];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);

    let raw = [
        24, 17, 49, 57, 56, 53, 49, 49, 48, 54, 50, 49, 48, 54, 50, 55, 46, 51, 90,
    ];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);

    let raw = [
        24, 21, 49, 57, 56, 53, 49, 49, 48, 54, 50, 49, 48, 54, 50, 55, 46, 51, 45, 48, 53, 48, 48,
    ];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);
}

#[test]
fn ia5_string() {
    let raw = [22, 9, 65, 66, 67, 68, 32, 69, 70, 71, 72];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);
}

#[test]
fn printable_string() {
    let raw = [19, 6, 80, 97, 114, 107, 101, 114];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);
}

#[test]
fn utc_time() {
    let raw = [23, 11, 57, 54, 48, 52, 49, 53, 50, 48, 51, 48, 90];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);

    let raw = [23, 13, 49, 57, 49, 48, 49, 55, 49, 55, 52, 49, 50, 56, 90];
    let asn1 = Asn1::decode_buff(&raw).unwrap();
    println!("{:?}", asn1);
}

// TODO: bug. need to be fixed
#[test]
fn oi() {
    let asn1 = Asn1Type::ObjectIdentifier(ObjectIdentifier::from(oid::ObjectIdentifier::try_from("2.29.1432919503.268680342.2607450773.2297838964.2800989460.3536442839.826751377.97234221.883516388.2427681722").unwrap()));
    println!("asn1: {:?}", asn1);
    let asn1_tag = asn1.tag();

    let buff_len = asn1.needed_buf_size();
    let mut buff = vec![0; buff_len];

    asn1.encode_buff(&mut buff).unwrap();
    println!("buff: {:?}", buff);

    let mut decoded = Asn1::decode_buff(&buff).unwrap();
    let decoded_inner_asn1 = decoded.inner_asn1();
    let decoded_meta = decoded.meta();

    println!("decoded_inner_asn1: {:?}", decoded_inner_asn1);

    assert_eq!(decoded_inner_asn1.needed_buf_size(), buff_len);
    assert_eq!(
        1 + decoded_meta.length_bytes().len() + decoded_meta.data_bytes().len(),
        buff_len
    );
    assert_eq!(decoded_inner_asn1.tag(), asn1_tag);
    assert_eq!(decoded_meta.tag_position(), 0);
    assert_eq!(decoded_meta.raw_bytes(), buff);

    decoded.clear_meta();
    assert_eq!(decoded.inner_asn1(), &asn1);
}

#[test]
fn decode_default() {
    let raw = &[
        48, 87, 1, 1, 255, 1, 1, 0, 160, 17, 12, 15, 84, 98, 101, 66, 101, 115, 116, 84, 118, 97, 114, 121, 110, 107,
        97, 161, 60, 48, 58, 5, 0, 164, 9, 4, 7, 48, 5, 160, 3, 1, 1, 255, 164, 7, 3, 5, 0, 64, 129, 0, 16, 164, 34,
        108, 32, 48, 30, 160, 2, 5, 0, 161, 24, 30, 22, 0, 67, 0, 101, 0, 114, 0, 116, 0, 105, 0, 102, 0, 105, 0, 99,
        0, 97, 0, 116, 0, 101,
    ];
    let asn1 = Asn1::decode_buff(raw).unwrap();
    println!("{:?}", asn1);
}

#[test]
fn decode_utc() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let raw = &[23, 13, 49, 56, 48, 55, 49, 54, 49, 52, 53, 54, 51, 53, 90];
    let asn1 = Asn1::decode_buff(raw).unwrap();

    let mut encoded = vec![0; asn1.needed_buf_size()];
    asn1.encode_buff(&mut encoded).expect("ASN1 encoding should not fail");
}
