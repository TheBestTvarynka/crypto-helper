#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub fn from(message: &'static str) -> Self {
        Self { message }
    }
}
