use std::{borrow::Borrow, fmt::Display};

use zip::result::ZipError;

/// The Error type returned by the dlprotoc crate.
///
/// This type only contains a message, so it is not expected to be handled.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    #[must_use]
    pub const fn from_string(message: String) -> Self {
        Self { message }
    }

    pub fn with_prefix(prefix: impl Borrow<str>, e: impl Display) -> Self {
        Self {
            message: format!("{}: {e}", prefix.borrow()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Self::with_prefix("zip error", e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::with_prefix("io error", e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        let message = if let Some(url) = e.url() {
            format!("failed downloading protoc from url: {url}: {e}")
        } else {
            format!("failed downloading protoc: {e}")
        };
        Self::from_string(message)
    }
}
