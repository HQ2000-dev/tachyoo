use crate::in_::types::string::McStringError;

pub struct McString<const MAX_LEN: u16> {
    data: Box<str>,
}

//TODO: move inside the module again?
pub const ABSOLUTE_MAX_LENGTH: u16 = 32767;

impl<const MAX_LENGTH: u16> McString<MAX_LENGTH> {
    const __ASSERTION: () = assert!(MAX_LENGTH <= ABSOLUTE_MAX_LENGTH);

    fn new(str: impl AsRef<str>) -> Result<u16, McStringError> {}

    pub fn len(&self) -> u16 {
        //invariant!!
        self.data.len() as u16
    }
}

impl<const MAX_LEN: u16> AsRef<str> for McString<MAX_LEN> {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

//TODO: collapse both impl into one function (mostly)
impl<const MAX_LENGTH: u16> TryFrom<String> for McString<MAX_LENGTH> {
    type Error = McStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let len = McString::<MAX_LENGTH>::is_valid_and_len(string.as_str())?;

        let len = VarInt::new(len.into());

        Ok(McString {
            len,
            data: string.into_boxed_str(),
        })
    }
}

impl<const MAX_LENGTH: u16> TryFrom<Box<str>> for McString<MAX_LENGTH> {
    type Error = McStringError;

    fn try_from(string: Box<str>) -> Result<Self, Self::Error> {
        let (valid, len) =
            crate::util::string::is_valid_and_len::<MAX_LENGTH, MAX_LENGTH>(&*string);

        if valid {
            Ok(McString { data: string })
        } else {
            Err(McStringError::TooLong { len })
        }
    }
}
