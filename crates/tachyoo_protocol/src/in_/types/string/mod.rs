pub mod str;
pub mod string;
pub mod parse;

use std::str::Utf8Error;

#[derive(Debug, Clone, thiserror::Error)]
#[error("Not a valid McString")]
pub enum McStringError {
    InvalidUtf8(Utf8Error),
    TooLong { len: u16 },
}

