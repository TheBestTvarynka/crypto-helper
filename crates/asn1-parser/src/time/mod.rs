mod generalized_time;
mod utc_time;

pub use generalized_time::{
    GeneralizedTime, LocalTimeDiffFactor, LocalTimeDirection, Second as GtSecond, Year as GtYear,
};
pub use utc_time::UtcTime;

use crate::Asn1Result;
use crate::reader::Reader;

macro_rules! define_nt {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name(u8);

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl AsRef<u8> for $name {
            fn as_ref(&self) -> &u8 {
                &self.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:02}", self.0)
            }
        }

        impl TryFrom<u8> for $name {
            type Error = crate::Error;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if value < 100 {
                    Ok($name(value))
                } else {
                    Err("invalid value".into())
                }
            }
        }
    };
}

define_nt!(Year);
define_nt!(Month);
define_nt!(Day);
define_nt!(Hour);
define_nt!(Minute);
define_nt!(Second);

fn read_number(reader: &mut Reader<'_>) -> Asn1Result<u8> {
    const ASCII_SHIFT: u8 = 48;

    let f = char::from(reader.read_byte()?);
    let s = char::from(reader.read_byte()?);

    if !f.is_numeric() || !s.is_numeric() {
        return Err("invalid bytes for utctime".into());
    }

    Ok((f as u8 - ASCII_SHIFT) * 10 + (s as u8 - ASCII_SHIFT))
}
