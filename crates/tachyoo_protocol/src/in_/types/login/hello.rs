use std::io::{self};

use tokio::io::AsyncReadExt;

use crate::in_::types::{
    UUID, parse_uuid,
    string::{
        parse::{parse_mc_string, unwrap_mc_string_parse_error},
        string::McString,
    },
};

#[derive(Debug)]
pub struct Hello {
    name: McString<16>,
    uuid: UUID,
}

pub async fn parse_hello<R: AsyncReadExt + Unpin>(reader: &mut R, len: usize) -> io::Result<Hello> {
    Ok(Hello {
        name: parse_mc_string(reader, len)
            .await
            .map_err(unwrap_mc_string_parse_error)?,
        uuid: parse_uuid(reader).await?,
    })
}
