use core::str::Utf8Error;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub fn from(message: &'static str) -> Self {
        Self { message }
    }
}

impl From<Utf8Error> for Error {
    fn from(_value: Utf8Error) -> Self {
        Self { message: "utf8error" }
    }
}
