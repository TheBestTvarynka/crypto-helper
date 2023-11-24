use alloc::string::FromUtf16Error;
use core::num::TryFromIntError;
use core::str::Utf8Error;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub fn message(&self) -> &str {
        self.message
    }
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Self {
        Self { message }
    }
}

impl From<Utf8Error> for Error {
    fn from(_value: Utf8Error) -> Self {
        Self { message: "Utf8error" }
    }
}

impl From<FromUtf16Error> for Error {
    fn from(_value: FromUtf16Error) -> Self {
        Self {
            message: "FromUtf16Error",
        }
    }
}

impl From<TryFromIntError> for Error {
    fn from(_value: TryFromIntError) -> Self {
        Self {
            message: "Numbers conversion error",
        }
    }
}
