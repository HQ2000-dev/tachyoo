use crate::in_::types::string::McStringError;

pub struct McStr<'a, const MAX_LENGTH: u16> {
    data: &'a str,
}

impl<'a, const MAX_LEN: u16> McStr<'a, MAX_LEN> {
    //TODO: fix
    pub const ABSOLUTE_MAX_LEN: u16 = 200000;

    fn new(bytes: &[u8]) -> Result<McStr<'_, MAX_LEN>, McStringError> {
        let str = str::from_utf8(bytes).map_err(McStringError::InvalidUtf8)?;

        let (valid, len) = crate::util::string::is_valid_and_len::<MAX_LEN, MAX_LEN>(str);

        if valid {
            Ok(McStr { data: str })
        } else {
            Err(McStringError::TooLong { len })
        }
    }
}
