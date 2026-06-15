use std::str::FromStr;

use crate::{in_::types::string::McStringError, util::string::ABSOLUTE_MAX_LEN};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct McString<const MAX_LEN: u16> {
    data: Box<str>,
}

//TODO: move inside the module again?

impl<const MAX_LEN: u16> McString<MAX_LEN> {
    const __ASSERTION: () = assert!(MAX_LEN <= ABSOLUTE_MAX_LEN);

    fn new(bytes: &[u8]) -> Result<McString<MAX_LEN>, McStringError> {
        let str = str::from_utf8(bytes).map_err(McStringError::InvalidUtf8)?;

        let (valid, len) = crate::util::string::is_valid_and_len::<MAX_LEN>(str);

        if valid {
            Ok(McString {
                //unwrap_infallible isn't stable yet
                data: String::from_str(str).unwrap().into_boxed_str(),
            })
        } else {
            Err(McStringError::TooLong { len })
        }
    }

    pub fn len(&self) -> u16 {
        // invariant!!
        self.data.len() as u16
    }
}

impl<const MAX_LEN: u16> AsRef<str> for McString<MAX_LEN> {
    fn as_ref(&self) -> &str {
        &self.data
    }
}
