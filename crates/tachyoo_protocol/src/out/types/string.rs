use tokio::io;

use crate::out::{Transfer, Writable, types::var::int::VarInt};

//better name!
pub struct McString<const MAX_LENGTH: u16> {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

impl<const MAX_LENGTH: u16> McString<MAX_LENGTH> {
    const __ASSERTION: () = assert!(MAX_LENGTH <= Self::ABSOLUTE_MAX_LENGTH);

    pub const ABSOLUTE_MAX_LENGTH: u16 = 32767;

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn is_valid_and_len(str: impl AsRef<str>) -> Result<u16, McStringError> {
        let str = str.as_ref();

        if str.len() * 2 > Self::ABSOLUTE_MAX_LENGTH as usize || str.len() * 2 > MAX_LENGTH as usize
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

#[async_trait::async_trait]
impl<const MAX_LENGTH: u16> Transfer for McString<MAX_LENGTH> {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.len.write_data(writeable).await?;
        writeable.write_all(self.data.as_bytes()).await?;

        Ok(())
    }
}
