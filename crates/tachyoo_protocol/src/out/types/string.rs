use crate::out::{Buffer, Transfer, types::var::int::VarInt};

//better name!
pub struct McString<const MAX_LENGTH: u16> {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

//TODO: move inside the module again?
pub const ABSOLUTE_MAX_LENGTH: u16 = 32767;

impl<const MAX_LENGTH: u16> McString<MAX_LENGTH> {
    const __ASSERTION: () = assert!(MAX_LENGTH <= ABSOLUTE_MAX_LENGTH);

    fn is_valid_and_len(str: impl AsRef<str>) -> Result<u16, McStringError> {
        let str = str.as_ref();

        if str.len() * 2 > ABSOLUTE_MAX_LENGTH as usize || str.len() * 2 > MAX_LENGTH as usize
        {
            return Err(McStringError(()));
        }

        let mut len = 0;

        for char in str.chars() {
            if char > '\u{FFFF}' {
                len += 2;
            } else {
                len += 1;
            }
        }

        if len > MAX_LENGTH {
            Err(McStringError(()))
        } else {
            Ok(len)
        }
    }

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
        let len = McString::<MAX_LENGTH>::is_valid_and_len(&*string)?;

        let len = VarInt::new(len.into());

        Ok(McString { len, data: string })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to convert into a McString")]
pub struct McStringError(());

impl<const MAX_LENGTH: u16> Transfer for McString<MAX_LENGTH> {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.len.write_bytes(buf);
        buf.write_all(self.data.as_bytes());
    }
}

pub type MaxLenMcString = McString<ABSOLUTE_MAX_LENGTH>;