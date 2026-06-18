use std::io;

use tokio::io::AsyncReadExt;

use crate::in_::types::{
    Int, UShort, parse_ushort,
    server_addr::{ServerAddr, parse_server_addr},
    var::{int::signed::parse_var_int, long::unsigned::parse_var_ulong},
};

#[derive(Debug)]

pub struct Handshake {
    protocol_version: Int,
    server_address: ServerAddr,
    server_port: UShort,
    next_state: Int,
}

pub async fn parse_handshake<R: AsyncReadExt + Unpin>(
    reader: &mut R,
    len: usize,
) -> io::Result<Handshake> {
    let (protocol_version, proto_len) = parse_var_int(reader).await?;

    //size_of::<u8> because the last varInt is one byte in length (FIXME)
    let server_addr_len = len - proto_len - size_of::<UShort>() - size_of::<u8>();
    let server_address = parse_server_addr(reader, server_addr_len).await?;

    let server_port = parse_ushort(reader).await?;

    let (next_state, next_state_len) = parse_var_int(reader).await?;

    assert_eq!(next_state_len, 1);

    Ok(Handshake {
        protocol_version,
        server_address,
        server_port,
        next_state,
    })
}
