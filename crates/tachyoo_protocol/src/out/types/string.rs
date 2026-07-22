use crate::{
    out::{Buffer, Transfer, types::var::int::VarInt},
    util::string::ABSOLUTE_MAX_LEN,
};

//better name!
#[derive(Debug)]
pub struct McString<const MAX_LEN: u16> {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

impl<const MAX_LEN: u16> McString<MAX_LEN> {
    const __ASSERTION: () = assert!(MAX_LEN <= ABSOLUTE_MAX_LEN);

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
impl<const MAX_LEN: u16> TryFrom<String> for McString<MAX_LEN> {
    type Error = McStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let (valid, len) = crate::util::string::is_valid_and_len::<MAX_LEN>(&*string);

        if valid {
            Ok(McString {
                len: VarInt::new(len.into()),
                data: string.into_boxed_str(),
            })
        } else {
            Err(McStringError(()))
        }
    }
}

impl<const MAX_LEN: u16> TryFrom<Box<str>> for McString<MAX_LEN> {
    type Error = McStringError;

    fn try_from(string: Box<str>) -> Result<Self, Self::Error> {
        let (valid, len) = crate::util::string::is_valid_and_len::<MAX_LEN>(&*string);

        if valid {
            Ok(McString {
                len: VarInt::new(len.into()),
                data: string,
            })
        } else {
            Err(McStringError(()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to convert into a McString")]
pub struct McStringError(());

impl<const MAX_LEN: u16> Transfer for McString<MAX_LEN> {
    fn write_bytes(&self, buf: &mut Buffer) {
        self.len.write_bytes(buf);
        buf.write_all(self.data.as_bytes());
    }
}

pub type MaxLenMcString = McString<ABSOLUTE_MAX_LEN>;
