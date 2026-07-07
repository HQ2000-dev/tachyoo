use std::{io, str::Utf8Error};

use tokio::io::AsyncReadExt;

use crate::in_::types::{string::{McStringError, str::McStr, string::McString}, var::int::signed::parse_var_int};

//TODO: convert to anyhow?
pub enum McStringParseError {
    Io(io::Error),
    McStringError(McStringError),
}

pub fn unwrap_mc_string_parse_error(err: McStringParseError) -> io::Error {
    match err {
        McStringParseError::Io(e) =>  e,
        McStringParseError::McStringError(e) => Err(e).unwrap(),
    }
}

pub async fn parse_mc_string<R: AsyncReadExt + Unpin, const MAX_LEN: u16>(
    reader: &mut R,
    len: usize,
) -> Result<McString<MAX_LEN>, McStringParseError>  {
    
    let (string_len, string_len_len) =parse_var_int(reader).await.map_err(McStringParseError::Io)?;
    
    assert!(McString::<MAX_LEN>::is_len_valid(string_len));

    //assert_eq!()
    
    McString::<MAX_LEN>::parse_inner(reader, string_len as u16).await
}

impl<const MAX_LEN: u16> McString<MAX_LEN> {
    async fn parse_inner<R: AsyncReadExt + Unpin>(reader: &mut R, len: u16) -> Result<Self, McStringParseError> {
        let mut buf=vec![0u8;len as usize].into_boxed_slice();
        reader.read_exact(&mut buf).await.map_err(McStringParseError::Io)?;
        //TODO: reuse the buffer allocation!
        McString::new(buf.as_ref()).map_err(McStringParseError::McStringError)
    }
}