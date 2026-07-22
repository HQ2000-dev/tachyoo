use std::io;

use tokio::io::AsyncReadExt;

pub mod array;
pub mod handshake;
pub mod login;
pub mod server_addr;
pub mod status;
pub mod string;
pub mod var;

pub type Long = i64;

pub type Int = i32;

pub type ULong = u64;

pub type UInt = u32;

pub type UShort = u16;

pub type UUID = u128;

pub type Byte = u8;

pub async fn parse_byte<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Byte> {
    let byte = reader.read_u8().await?.to_le();
    Ok(byte)
}

pub async fn parse_ushort<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<UShort> {
    let ushort = reader.read_u16().await?.to_le();
    Ok(ushort)
}

pub async fn parse_uuid<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<UUID> {
    let uuid = reader.read_u128().await?.to_le();
    Ok(uuid)
}

pub async fn parse_long<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Long> {
    let long = reader.read_i64().await?.to_le();
    Ok(long)
}
