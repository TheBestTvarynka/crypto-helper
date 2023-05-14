use std::fmt;

use picky::key::{PrivateKey, PublicKey};
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{de, Deserializer, Serializer};

pub fn serialize_bytes<S>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&hex::encode(bytes))
}

pub fn deserialize_bytes<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("hex encoded bytes")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            hex::decode(v).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    d.deserialize_str(Visitor)
}

pub fn serialize_private_key<S>(private_key: &PrivateKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pem_str = private_key.to_pem_str().unwrap();
    s.serialize_str(&pem_str)
}

pub fn deserialize_private_key<'de, D>(d: D) -> Result<PrivateKey, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = PrivateKey;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("pem encoded private key")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            PrivateKey::from_pem_str(v).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    d.deserialize_str(Visitor)
}

pub fn serialize_public_key<S>(public_key: &PublicKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pem_str = public_key.to_pem_str().unwrap();
    s.serialize_str(&pem_str)
}

pub fn deserialize_public_key<'de, D>(d: D) -> Result<PublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = PublicKey;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("pem encoded private key")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            PublicKey::from_pem_str(v).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    d.deserialize_str(Visitor)
}

pub fn serialize_rsa_private_key<S>(private_key: &RsaPrivateKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pem_str = private_key.to_pkcs1_pem(Default::default()).unwrap();
    s.serialize_str(&pem_str)
}

pub fn deserialize_rsa_private_key<'de, D>(d: D) -> Result<RsaPrivateKey, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = RsaPrivateKey;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("pem encoded private key")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            RsaPrivateKey::from_pkcs1_pem(v).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    d.deserialize_str(Visitor)
}

pub fn serialize_rsa_public_key<S>(public_key: &RsaPublicKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let pem_str = public_key.to_pkcs1_pem(Default::default()).unwrap();
    s.serialize_str(&pem_str)
}

pub fn deserialize_rsa_public_key<'de, D>(d: D) -> Result<RsaPublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = RsaPublicKey;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("pem encoded private key")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            RsaPublicKey::from_pkcs1_pem(v).map_err(|err| de::Error::custom(err.to_string()))
        }
    }

    d.deserialize_str(Visitor)
}
