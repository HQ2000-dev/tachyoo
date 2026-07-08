use std::io;

use tokio::io::AsyncReadExt;

use crate::in_::types::array::{PrefixedByteArray, parse_prefixed_byte_array};

#[derive(Debug)]
pub struct Key {
    shared_secret: PrefixedByteArray,
    verify_token: PrefixedByteArray,
}

pub async fn parse_key<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Key> {
    Ok(
        Key {
            shared_secret: parse_prefixed_byte_array(reader).await?,
            verify_token: parse_prefixed_byte_array(reader).await?,
        }
    )
}