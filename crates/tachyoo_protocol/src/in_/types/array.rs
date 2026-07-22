use std::io;

use tokio::io::AsyncReadExt;

use crate::in_::types::{Byte, parse_byte, var::int::signed::parse_var_int};
/*
pub struct Array<T>(Box<[T]>);

async fn parse_array<R: AsyncReadExt + Unpin>(reader: &mut R, len: usize) -> io::Result<Array<T>> {

}*/

//workaround until the trait abstractions are finished
#[derive(Debug)]
pub struct PrefixedByteArray(Box<[Byte]>);

pub async fn parse_prefixed_byte_array<R: AsyncReadExt + Unpin>(
    reader: &mut R,
) -> io::Result<PrefixedByteArray> {
    let (array_len, _array_len_len) = parse_var_int(reader).await?;

    let mut buf = Vec::with_capacity(array_len as usize);

    for _ in 0..array_len {
        buf.push(parse_byte(reader).await?);
    }

    Ok(PrefixedByteArray(buf.into_boxed_slice()))
}
