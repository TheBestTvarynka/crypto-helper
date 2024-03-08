use std::fmt::{Display, Formatter};
use std::ops::Deref;

use similar::Algorithm;

const MYERS: &str = "Myers";
const PATIENCE: &str = "Patience";
const LCS: &str = "Lcs";

#[derive(Clone, Copy, Eq, PartialEq)]
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

impl Display for DiffAlgo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            Algorithm::Myers => MYERS,
            Algorithm::Patience => PATIENCE,
            Algorithm::Lcs => LCS,
        })
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
