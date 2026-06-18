use std::io;

use tokio::io::AsyncReadExt;

pub mod handshake;
pub mod server_addr;
pub mod string;
pub mod var;

pub type Long = i64;

pub type Int = i32;

pub type ULong = u64;

pub type UInt = u32;

pub type UShort = u16;

pub async fn parse_ushort<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<UShort> {
    let ushort = reader.read_u16().await?.to_le();
    Ok(ushort)
}
