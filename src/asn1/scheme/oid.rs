use asn1_parser::{Mutable, ObjectIdentifier, RawAsn1EntityData};
use yew::{Html, Properties, function_component, html};

use crate::asn1::node_options::NodeOptions;
use crate::common::RcSlice;

#[derive(PartialEq, Properties, Clone)]
pub struct ObjectIdentifierProps {
    pub node: Mutable<ObjectIdentifier>,
    pub meta: RawAsn1EntityData,
}

#[function_component(ObjectIdentifierNode)]
pub fn bool(props: &ObjectIdentifierProps) -> Html {
    let offset = props.meta.tag_position();
    let length_len = props.meta.length_range().len();
    let data_len = props.meta.data_range().len();

    let formatted = props.node.get().format();

    html! {
        <div class="terminal-asn1-node">
            <NodeOptions node_bytes={RcSlice::from(props.meta.raw_bytes())} {offset} {length_len} {data_len} name={String::from("Object Identifier")}/>
            <span class="asn-simple-value">{&formatted}</span>
            {{
                let (name, url) = oid_name(&formatted);
                if !name.is_empty() { html! {
                    <a class="a-link" href={url}>{name}</a>
                }} else {html! {}}
            }}
        </div>
    }
}

fn oid_name(oid: &'_ str) -> (&'static str, &'static str) {
    match oid {
        "1.2.840.113549.1.1.1" => ("rsaEncryption", "http://www.oid-info.com/get/1.2.840.113549.1.1.1"),
        "1.2.840.10040.4.3" => ("id-dsa-with-sha1", "http://www.oid-info.com/get/1.2.840.10040.4.3"),
        "1.2.840.10046.2.1" => ("dh-public-number", "http://www.oid-info.com/get/1.2.840.10046.2.1"),
        "1.2.840.10045.2.1" => ("id-ecPublicKey", "http://www.oid-info.com/get/1.2.840.10045.2.1"),
        "1.2.840.10045.4.3.2" => ("ecdsa-with-SHA256", "http://www.oid-info.com/get/1.2.840.10045.4.3.2"),
        "1.2.840.10045.4.3.3" => ("ecdsa-with-SHA384", "http://www.oid-info.com/get/1.2.840.10045.4.3.3"),
        "1.2.840.10045.4.3.4" => ("ecdsa-with-SHA512", "http://www.oid-info.com/get/1.2.840.10045.4.3.4"),
        "1.2.840.10045.3.1.1" => (
            "prime192v1 (secp192r1)",
            "http://www.oid-info.com/get/1.2.840.10045.3.1.1",
        ),
        "1.2.840.10045.3.1.7" => (
            "prime256v1 (secp256r1)",
            "http://www.oid-info.com/get/1.2.840.10045.3.1.7",
        ),
        "1.2.840.113549.1.1.4" => (
            "md5WithRSAEncryption",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.4",
        ),
        "1.2.840.113549.1.1.5" => (
            "sha1-with-rsa-signature",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.5",
        ),
        "1.2.840.113549.1.1.11" => (
            "sha256-with-rsa-signature",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.11",
        ),
        "1.2.840.113549.1.1.12" => (
            "sha384-with-rsa-signature",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.12",
        ),
        "1.2.840.113549.1.1.13" => (
            "sha512-with-rsa-signature",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.13",
        ),
        "1.2.840.113549.1.1.14" => (
            "sha224-with-rsa-signature",
            "http://www.oid-info.com/get/1.2.840.113549.1.1.14",
        ),
        "1.2.840.113549.1.1.10" => ("rsassa-pss", "http://www.oid-info.com/get/1.2.840.113549.1.1.10"),
        "1.2.840.113549.1.9.1" => (
            "pkcs-9-at-emailAddress",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.1",
        ),
        "1.2.840.113549.1.9.14" => (
            "pkcs-9-at-extensionRequest",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.14",
        ),
        "1.2.840.113549.1.7.1" => ("id-data", "http://www.oid-info.com/get/1.2.840.113549.1.7.1"),
        "1.2.840.113549.1.7.6" => ("id-encryptedData", "http://www.oid-info.com/get/1.2.840.113549.1.7.6"),
        "1.2.840.113549.1.7.2" => ("id-signedData", "http://www.oid-info.com/get/1.2.840.113549.1.7.2"),
        "1.2.840.113549.1.9.3" => (
            "pkcs-9-at-contentType",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.3",
        ),
        "1.2.840.113549.1.9.4" => ("id-messageDigest", "http://www.oid-info.com/get/1.2.840.113549.1.9.4"),
        "2.16.840.1.101.3.4.3.1" => ("dsa-with-sha224", "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.1"),
        "2.16.840.1.101.3.4.3.2" => ("dsa-with-sha256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.2"),
        "2.16.840.1.101.3.4.3.3" => ("dsa-with-sha384", "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.3"),
        "2.16.840.1.101.3.4.3.4" => ("dsa-with-sha256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.4"),
        "2.16.840.1.101.3.4.3.10" => (
            "id-ecdsa-with-sha3-256",
            "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.10",
        ),
        "2.16.840.1.101.3.4.3.13" => (
            "id-rsassa-pkcs1-v1-5-with-sha3-224",
            "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.13",
        ),
        "2.16.840.1.101.3.4.3.14" => (
            "id-rsassa-pkcs1-v1-5-with-sha3-256",
            "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.14",
        ),
        "2.16.840.1.101.3.4.3.15" => (
            "id-rsassa-pkcs1-v1-5-with-sha3-384",
            "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.15",
        ),
        "2.16.840.1.101.3.4.3.16" => (
            "id-rsassa-pkcs1-v1-5-with-sha3-512",
            "http://www.oid-info.com/get/2.16.840.1.101.3.4.3.16",
        ),
        "1.3.132.0.34" => ("ansip384r1 (secp384r1)", "http://www.oid-info.com/get/1.3.132.0.34"),
        "1.3.132.0.1" => ("ansit163k1 (sect163k1)", "http://www.oid-info.com/get/1.3.132.0.1"),
        "1.3.132.0.15" => ("ansit163r2 (sect163r2)", "http://www.oid-info.com/get/1.3.132.0.15"),
        "1.3.132.0.33" => ("ansip224r1 (secp224r1)", "http://www.oid-info.com/get/1.3.132.0.33"),
        "1.3.132.0.26" => ("ansit233k1 (sect233k1)", "http://www.oid-info.com/get/1.3.132.0.26"),
        "1.3.132.0.27" => ("ansit233r1 (sect233r1)", "http://www.oid-info.com/get/1.3.132.0.27"),
        "1.3.132.0.16" => ("ansit283k1 (sect283k1)", "http://www.oid-info.com/get/1.3.132.0.16"),
        "1.3.132.0.17" => ("ansit283r1 (sect283r1)", "http://www.oid-info.com/get/1.3.132.0.17"),
        "1.3.132.0.36" => ("ansit409k1 (sect409k1)", "http://www.oid-info.com/get/1.3.132.0.36"),
        "1.3.132.0.37" => ("ansit409r1 (sect409r1)", "http://www.oid-info.com/get/1.3.132.0.37"),
        "1.3.132.0.35" => ("ansip521r1 (secp521r1)", "http://www.oid-info.com/get/1.3.132.0.35"),
        "1.3.132.0.38" => ("ansit571k1 (sect571k1)", "http://www.oid-info.com/get/1.3.132.0.38"),
        "1.3.132.0.39" => ("ansit571r1 (sect571r1)", "http://www.oid-info.com/get/1.3.132.0.39"),
        "1.3.101.110" => ("id-X25519", "http://www.oid-info.com/get/1.3.101.110"),
        "1.3.101.111" => ("id-X448", "http://www.oid-info.com/get/1.3.101.111"),
        "1.3.101.112" => ("id-Ed25519", "http://www.oid-info.com/get/1.3.101.112"),
        "1.3.101.113" => ("id-Ed448", "http://www.oid-info.com/get/1.3.101.113"),
        "1.3.6.1.5.5.7.3.1" => ("id-kp-serverAuth", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.1"),
        "1.3.6.1.5.5.7.3.2" => ("id-kp-clientAuth", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.2"),
        "1.3.6.1.5.5.7.3.3" => ("id-kp-codeSigning", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.3"),
        "1.3.6.1.5.5.7.3.4" => ("id-kp-emailProtection", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.4"),
        "1.3.6.1.5.5.7.3.5" => ("id-kp-ipsecEndSystem", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.5"),
        "1.3.6.1.5.5.7.3.6" => ("id-kp-ipsecTunnel", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.6"),
        "1.3.6.1.5.5.7.3.7" => ("id-kp-ipsecUser", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.7"),
        "1.3.6.1.5.5.7.3.8" => ("id-kp-timeStamping", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.8"),
        "1.3.6.1.5.5.7.3.9" => ("id-kp-OCSPSigning", "http://www.oid-info.com/get/1.3.6.1.5.5.7.3.9"),
        "2.5.29.37.0" => ("anyExtendedKeyUsage", "http://www.oid-info.com/get/2.5.29.37.0"),
        "1.3.6.1.4.1.311.10.3.13" => (
            "szOID_KP_LIFETIME_SIGNING",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.3.13",
        ),
        "2.5.4.3" => ("commonName", "http://www.oid-info.com/get/2.5.4.3"),
        "2.5.4.4" => ("surname", "http://www.oid-info.com/get/2.5.4.4"),
        "2.5.4.5" => ("serialNumber", "http://www.oid-info.com/get/2.5.4.5"),
        "2.5.4.6" => ("countryName", "http://www.oid-info.com/get/2.5.4.6"),
        "2.5.4.7" => ("localityName", "http://www.oid-info.com/get/2.5.4.7"),
        "2.5.4.8" => ("stateOrProvinceName", "http://www.oid-info.com/get/2.5.4.8"),
        "2.5.4.9" => ("streetAddress", "http://www.oid-info.com/get/2.5.4.9"),
        "2.5.4.10" => ("organizationName", "http://www.oid-info.com/get/2.5.4.10"),
        "2.5.4.11" => ("organizationalUnitName", "http://www.oid-info.com/get/2.5.4.11"),
        "2.5.4.42" => ("givenName", "http://www.oid-info.com/get/2.5.4.42"),
        "2.5.4.20" => ("telephoneNumber", "http://www.oid-info.com/get/2.5.4.20"),
        "2.5.29.14" => ("subjectKeyIdentifier", "http://www.oid-info.com/get/2.5.29.14"),
        "2.5.29.15" => ("keyUsage", "http://www.oid-info.com/get/2.5.29.15"),
        "2.5.29.17" => ("subjectAltName", "http://www.oid-info.com/get/2.5.29.17"),
        "2.5.29.18" => ("issuerAltName", "http://www.oid-info.com/get/2.5.29.18"),
        "2.5.29.19" => ("basicConstraints", "http://www.oid-info.com/get/2.5.29.19"),
        "2.5.29.20" => ("cRLNumber", "http://www.oid-info.com/get/2.5.29.20"),
        "2.5.29.35" => ("authorityKeyIdentifier", "http://www.oid-info.com/get/2.5.29.35"),
        "2.5.29.37" => ("extKeyUsage", "http://www.oid-info.com/get/2.5.29.37"),
        "2.16.840.1.101.3.4.1.1" => ("aes128-ECB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.1"),
        "2.16.840.1.101.3.4.1.2" => ("aes128-CBC-PAD", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.2"),
        "2.16.840.1.101.3.4.1.3" => ("aes128-OFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.3"),
        "2.16.840.1.101.3.4.1.4" => ("aes128-CFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.4"),
        "2.16.840.1.101.3.4.1.5" => ("aes128-wrap", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.5"),
        "2.16.840.1.101.3.4.1.6" => ("aes128-GCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.6"),
        "2.16.840.1.101.3.4.1.7" => ("aes128-CCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.7"),
        "2.16.840.1.101.3.4.1.8" => ("aes128-wrap-pad", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.8"),
        "2.16.840.1.101.3.4.1.21" => ("aes192-ECB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.21"),
        "2.16.840.1.101.3.4.1.22" => ("aes192-CBC-PAD", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.22"),
        "2.16.840.1.101.3.4.1.23" => ("aes192-OFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.23"),
        "2.16.840.1.101.3.4.1.24" => ("aes192-CFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.24"),
        "2.16.840.1.101.3.4.1.25" => ("aes192-wrap", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.25"),
        "2.16.840.1.101.3.4.1.26" => ("aes192-GCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.26"),
        "2.16.840.1.101.3.4.1.27" => ("aes192-CCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.27"),
        "2.16.840.1.101.3.4.1.28" => ("aes192-wrap-pad", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.28"),
        "2.16.840.1.101.3.4.1.41" => ("aes256-ECB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.41"),
        "2.16.840.1.101.3.4.1.42" => ("aes256-CBC-PAD", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.42"),
        "2.16.840.1.101.3.4.1.43" => ("aes256-OFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.43"),
        "2.16.840.1.101.3.4.1.44" => ("aes256-CFB", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.44"),
        "2.16.840.1.101.3.4.1.45" => ("id-aes256-wrap", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.45"),
        "2.16.840.1.101.3.4.1.46" => ("aes256-GCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.46"),
        "2.16.840.1.101.3.4.1.47" => ("aes256-CCM", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.47"),
        "2.16.840.1.101.3.4.1.48" => ("aes256-wrap-pad", "http://www.oid-info.com/get/2.16.840.1.101.3.4.1.48"),
        "1.2.840.113549.2" => ("digestAlgorithm", "http://www.oid-info.com/get/1.2.840.113549.2"),
        "1.2.840.113549.2.5" => ("md5", "http://www.oid-info.com/get/1.2.840.113549.2.5"),
        "1.2.840.113549.2.7" => ("hmacWithSHA1", "http://www.oid-info.com/get/1.2.840.113549.2.7"),
        "1.2.840.113549.2.8" => ("hmacWithSHA224", "http://www.oid-info.com/get/1.2.840.113549.2.8"),
        "1.2.840.113549.2.9" => ("hmacWithSHA256", "http://www.oid-info.com/get/1.2.840.113549.2.9"),
        "1.2.840.113549.2.10" => ("hmacWithSHA384", "http://www.oid-info.com/get/1.2.840.113549.2.10"),
        "1.2.840.113549.2.11" => ("hmacWithSHA512", "http://www.oid-info.com/get/1.2.840.113549.2.11"),
        "1.3.14.3.2.26" => ("hashAlgorithmIdentifier", "http://www.oid-info.com/get/1.3.14.3.2.26"),
        "2.16.840.1.101.3.4.2.1" => ("sha256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.1"),
        "2.16.840.1.101.3.4.2.2" => ("sha384", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.2"),
        "2.16.840.1.101.3.4.2.3" => ("sha512", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.3"),
        "2.16.840.1.101.3.4.2.4" => ("sha224", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.4"),
        "2.16.840.1.101.3.4.2.5" => ("sha512-224", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.5"),
        "2.16.840.1.101.3.4.2.6" => ("sha512-256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.6"),
        "2.16.840.1.101.3.4.2.7" => ("sha3-224", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.7"),
        "2.16.840.1.101.3.4.2.8" => ("sha3-256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.8"),
        "2.16.840.1.101.3.4.2.9" => ("sha3-384", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.9"),
        "2.16.840.1.101.3.4.2.10" => ("sha3-512", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.10"),
        "2.16.840.1.101.3.4.2.11" => ("id-shake128", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.11"),
        "2.16.840.1.101.3.4.2.12" => ("id-shake256", "http://www.oid-info.com/get/2.16.840.1.101.3.4.2.12"),
        "1.2.840.113549.1.9.5" => (
            "pkcs-9-at-signingTime",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.5",
        ),
        "1.2.840.113549.1.9.6" => (
            "pkcs-9-at-counterSignature",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.6",
        ),
        "1.3.6.1.4.1.311.2.1.4" => (
            "SPC_INDIRECT_DATA_OBJID ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.1.4",
        ),
        "1.3.6.1.4.1.311.2.1.11" => (
            "SPC_STATEMENT_TYPE_OBJID ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.1.11",
        ),
        "1.3.6.1.4.1.311.2.1.12" => (
            "SPC_SP_OPUS_INFO_OBJID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.1.12",
        ),
        "1.3.6.1.4.1.311.2.1.15" => (
            "SPC_PE_IMAGE_DATA_OBJID ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.1.15",
        ),
        "1.3.6.1.4.1.311.2.1.30" => (
            "SPC_SIPINFO_OBJID ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.1.30",
        ),
        "1.3.6.1.4.1.311.3.2.1" => (
            "SPC_TIME_STAMP_REQUEST_OBJID ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.3.2.1",
        ),
        "1.3.6.1.4.1.311.3.3.1" => (
            "Timestamping signature (Ms-CounterSign)",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.3.3.1",
        ),
        "1.3.6.1.4.1.311.10.1" => ("szOID_CTL", "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.1"),
        "1.3.6.1.4.1.311.10.3.9" => (
            "szOID_ROOT_LIST_SIGNER ",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.3.9",
        ),
        "1.3.6.1.4.1.311.10.11.9" => (
            "CERT_ENHKEY_USAGE_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.9",
        ),
        "1.3.6.1.4.1.311.10.11.11" => (
            "CERT_FRIENDLY_NAME_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.11",
        ),
        "1.3.6.1.4.1.311.10.11.20" => (
            "certKeyIdentifierPropId",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.20",
        ),
        "1.3.6.1.4.1.311.10.11.29" => (
            "certSubjectNameMd5HashPropId",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.29",
        ),
        "1.3.6.1.4.1.311.10.11.83" => (
            "CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.83",
        ),
        "1.3.6.1.4.1.311.10.11.98" => (
            "CERT_AUTH_ROOT_SHA256_HASH_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.98",
        ),
        "1.3.6.1.4.1.311.10.11.104" => (
            "CERT_DISALLOWED_FILETIME_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.104",
        ),
        "1.3.6.1.4.1.311.10.11.105" => (
            "CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.10.11.105",
        ),
        "1.3.6.1.4.1.311.10.11.122" => (
            "DISALLOWED_ENHKEY_USAGE",
            "https://github.com/ralphje/signify/issues/12",
        ),
        "1.3.6.1.4.1.311.10.11.126" => (
            "CERT_NOT_BEFORE_FILETIME_PROP_ID",
            "https://www.frankysweb.de/kostenloses-s-mime-zertifikat-update-april-2020",
        ),
        "1.3.6.1.4.1.311.10.11.127" => (
            "http://127.0.0.1:8080CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID",
            "https://www.frankysweb.de/kostenloses-s-mime-zertifikat-update-april-2020",
        ),
        "1.3.6.1.4.1.311.60.3.2" => (
            "Auto Update End Revocation",
            "https://www.powershellgallery.com/packages/AutomatedLab.Common/1.1.5/Content/PkiHelper%5CPublic%5CNew-CaTemplate.ps1",
        ),
        "1.2.840.113549.1.1.8" => ("id-mgf1", "http://www.oid-info.com/get/1.2.840.113549.1.1.8"),
        "1.2.840.113554.1.2.2" => ("krb5", "http://www.oid-info.com/get/1.2.840.113554.1.2.2"),
        "1.2.840.48018.1.2.2" => (
            "MS-KILE",
            "https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-kile/829b9629-21ab-474f-8716-77cc0990aeb4",
        ),
        "1.2.840.113554.1.2.2.3" => (
            "user-to-user-mechanism",
            "http://www.oid-info.com/get/1.2.840.113554.1.2.2.3",
        ),
        "1.3.6.1.4.1.311.2.2.10" => ("NTLM", "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.2.10"),
        "1.3.6.1.4.1.311.2.2.30" => ("NEGOEX", "http://www.oid-info.com/get/1.3.6.1.4.1.311.2.2.30"),
        "1.3.6.1.5.5.2" => ("snego", "http://www.oid-info.com/get/1.3.6.1.5.5.2"),
        "1.3.6.1.5.2.7" => ("GSS PKU2U ", "https://oidref.com/1.3.6.1.5.2.7"),
        "1.3.6.1.5.2.3.1" => ("id-pkinit-authData", "http://www.oid-info.com/get/1.3.6.1.5.2.3.1"),
        "1.3.6.1.5.2.3.2" => ("id-pkinit-DHKeyData", "http://www.oid-info.com/get/1.3.6.1.5.2.3.2"),
        "1.2.840.113549.1.12.1.3" => (
            "pbeWithSHAAnd3-KeyTripleDES-CBC",
            "http://www.oid-info.com/get/1.2.840.113549.1.12.1.3",
        ),
        "1.2.840.113549.1.12.1.6" => (
            "pbeWithSHAAnd40BitRC2-CBC",
            "http://www.oid-info.com/get/1.2.840.113549.1.12.1.6",
        ),
        "1.2.840.113549.1.9.23.1" => ("x509Crl", "http://www.oid-info.com/get/1.2.840.113549.1.9.23.1"),
        "1.2.840.113549.1.9.22.1" => ("x509Certificate", "http://www.oid-info.com/get/1.2.840.113549.1.9.22.1"),
        "1.2.840.113549.1.12.10.1.1" => ("keyBag", "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.1"),
        "1.2.840.113549.1.12.10.1.2" => (
            "pkcs-8ShroudedKeyBag",
            "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.2",
        ),
        "1.2.840.113549.1.12.10.1.3" => ("certBag", "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.3"),
        "1.2.840.113549.1.12.10.1.4" => ("crlBag", "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.4"),
        "1.2.840.113549.1.12.10.1.5" => ("secretBag", "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.5"),
        "1.2.840.113549.1.12.10.1.6" => (
            "safeContentsBag",
            "http://www.oid-info.com/get/1.2.840.113549.1.12.10.1.6",
        ),
        "1.2.840.113549.1.5.12" => ("id-PBKDF2", "http://www.oid-info.com/get/1.2.840.113549.1.5.12"),
        "1.2.840.113549.1.5.13" => ("pbes2", "http://www.oid-info.com/get/1.2.840.113549.1.5.13"),
        "1.2.840.113549.1.9.20" => (
            "pkcs-9-at-friendlyName",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.20",
        ),
        "1.2.840.113549.1.9.21" => (
            "pkcs-9-at-localKeyId",
            "http://www.oid-info.com/get/1.2.840.113549.1.9.21",
        ),
        "1.3.6.1.4.1.311.20.2.3" => (
            "User Principal Name",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.20.2.3",
        ),
        "1.2.840.113549.1.9.16.2.47" => (
            "Signing certificate V2",
            "http://oid-info.com/get/1.2.840.113549.1.9.16.2.47",
        ),
        "1.2.840.113549.1.9.52" => (
            "id-aa-CMSAlgorithmProtection",
            "https://oidref.com/1.2.840.113549.1.9.52",
        ),
        "2.5.29.31" => (
            "cRLDistributionPoints (X509 extension)",
            "http://www.oid-info.com/get/2.5.29.31",
        ),
        "1.3.6.1.5.5.7.2.1" => (
            "PKIX CPS pointer qualifier",
            "http://www.oid-info.com/get/1.3.6.1.5.5.7.2.1",
        ),
        "1.3.6.1.4.1.44947.1.1.1" => (
            "ISRG Domain Validated (by Let's Encrypt)",
            "https://www.alvestrand.no/objectid/submissions/1.3.6.1.4.1.44947.1.1.1.html",
        ),
        "2.23.140.1.2.1" => ("domain-validated", "http://www.oid-info.com/get/2.23.140.1.2.1"),
        "2.5.29.32" => ("id-ce-certificatePolicies", "http://www.oid-info.com/get/2.5.29.32"),
        "1.3.6.1.5.5.7.48.2" => ("id-ad-caIssuers", "http://www.oid-info.com/get/1.3.6.1.5.5.7.48.2"),
        "1.3.6.1.5.5.7.48.1" => ("id-pkix-ocsp", "http://www.oid-info.com/get/1.3.6.1.5.5.7.48.1"),
        "1.3.6.1.5.5.7.1.1" => (
            "id-pe-authorityInfoAccess",
            "http://www.oid-info.com/get/1.3.6.1.5.5.7.1.1",
        ),
        "2.23.140.1.2.2" => ("organization-validated", "http://www.oid-info.com/get/2.23.140.1.2.2"),
        "1.3.6.1.4.1.11129.2.4.2" => (
            "Rec. ITU-T X.509v3 certificate extension",
            "http://www.oid-info.com/get/1.3.6.1.4.1.11129.2.4.2",
        ),
        "2.23.140.1.1" => ("ev-guidelines", "http://www.oid-info.com/get/2.23.140.1.1"),
        "2.23.140.1.2.3" => ("individual-validated", "http://www.oid-info.com/get/2.23.140.1.2.3"),
        "0.9.2342.19200300.100.1.25" => (
            "domainComponent",
            "http://www.oid-info.com/get/0.9.2342.19200300.100.1.25",
        ),
        "1.3.6.1.4.1.311.21.7" => (
            "szOID_CERTIFICATE_TEMPLATE",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.21.7",
        ),
        "1.3.6.1.4.1.311.20.2.2" => (
            "Smartcard logon (Microsoft enhanced key usage)",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.20.2.2",
        ),
        "1.3.6.1.4.1.311.21.10" => (
            "szOID_APPLICATION_CERT_POLICIES",
            "http://www.oid-info.com/get/1.3.6.1.4.1.311.21.10",
        ),
        "1.2.840.113549.1.7.3" => ("id-envelopedData", "http://oid-info.com/get/1.2.840.113549.1.7.3"),
        "1.3.6.1.4.1.311.74.1" => (
            "ProtectionDescriptorType",
            "https://github.com/jborean93/dpapi-ng/blob/57143c31897e647d97f5a8b505188dc447025997/src/dpapi_ng/_blob.py#L128",
        ),
        "1.3.6.1.4.1.311.74.1.1" => (
            "SID Protection Descriptor",
            "https://github.com/jborean93/dpapi-ng/blob/57143c31897e647d97f5a8b505188dc447025997/src/dpapi_ng/_blob.py#L129",
        ),
        "1.3.6.1.4.1.311.74.1.2" => (
            "Key File Protection Descriptor",
            "https://github.com/jborean93/dpapi-ng/blob/57143c31897e647d97f5a8b505188dc447025997/src/dpapi_ng/_blob.py#L130",
        ),
        "1.3.6.1.4.1.311.74.1.5" => (
            "SSDL Protection Descriptor",
            "https://github.com/jborean93/dpapi-ng/blob/57143c31897e647d97f5a8b505188dc447025997/src/dpapi_ng/_blob.py#L131",
        ),
        "1.3.6.1.4.1.311.74.1.8" => (
            "LOCAL Protection Descriptor",
            "https://github.com/jborean93/dpapi-ng/blob/57143c31897e647d97f5a8b505188dc447025997/src/dpapi_ng/_blob.py#L132",
        ),
        "1.3.6.1.4.1.311.21.1" => (
            "Certificate services CA version",
            "http://oid-info.com/get/1.3.6.1.4.1.311.21.1",
        ),
        "1.3.6.1.4.1.41482.3.3" => (
            "YubiKey firmware version",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        "1.3.6.1.4.1.41482.3.7" => (
            "YubiKey serial number",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        "1.3.6.1.4.1.41482.3.8" => (
            "YubiKey pin policy + touch policy",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        "1.3.6.1.4.1.41482.3.9" => (
            "YubiKey formfactor",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        "1.3.6.1.4.1.41482.3.10" => (
            "FIPS Certified YubiKey",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        "1.3.6.1.4.1.41482.3.11" => (
            "CSPN Certified YubiKey",
            "https://developers.yubico.com/PIV/Introduction/PIV_attestation.html",
        ),
        _ => ("-", "https://github.com/TheBestTvarynka/crypto-helper/issues/new"),
    }
}
