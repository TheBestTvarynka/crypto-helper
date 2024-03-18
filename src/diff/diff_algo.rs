use std::fmt::{Display, Formatter};
use std::ops::Deref;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use similar::Algorithm;

const MYERS: &str = "Myers";
const PATIENCE: &str = "Patience";
const LCS: &str = "Lcs";

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct DiffAlgo(pub Algorithm);

impl From<DiffAlgo> for Algorithm {
    fn from(value: DiffAlgo) -> Self {
        value.0
    }
}

impl From<Algorithm> for DiffAlgo {
    fn from(value: Algorithm) -> Self {
        Self(value)
    }
}

impl AsRef<str> for DiffAlgo {
    fn as_ref(&self) -> &str {
        match self.0 {
            Algorithm::Myers => MYERS,
            Algorithm::Patience => PATIENCE,
            Algorithm::Lcs => LCS,
        }
    }
}

impl Display for DiffAlgo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Deref for DiffAlgo {
    type Target = Algorithm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for DiffAlgo {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            MYERS => Algorithm::Myers.into(),
            PATIENCE => Algorithm::Patience.into(),
            LCS => Algorithm::Lcs.into(),
            _ => return Err(format!("Unsupported diff algorithm: {}.", value)),
        })
    }
}

impl<'de> Deserialize<'de> for DiffAlgo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DiffAlgoVisitor;

        impl Visitor<'_> for DiffAlgoVisitor {
            type Value = DiffAlgo;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("valid DiffAlgo")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                DiffAlgo::try_from(v).map_err(|err| E::custom(format!("Can not deserialize DiffAlgo: {:?}", err)))
            }
        }

        deserializer.deserialize_str(DiffAlgoVisitor)
    }
}

impl Serialize for DiffAlgo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}
