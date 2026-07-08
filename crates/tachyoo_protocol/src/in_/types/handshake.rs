use std::{convert::Infallible, io};

use tokio::io::AsyncReadExt;

use crate::in_::{types::{
    Int, UShort, parse_ushort,
    server_addr::{ServerAddr, parse_server_addr},
    var::{int::signed::parse_var_int, long::unsigned::parse_var_ulong},
}};

#[derive(Debug)]

pub struct Handshake {
    pub protocol_version: Int,
    pub server_address: ServerAddr,
    pub server_port: UShort,
    pub intent: Intent,
}


    pub async fn parse_handshake<R: AsyncReadExt + Unpin>(reader: &mut R, len: usize) -> io::Result<Handshake> {
        let (protocol_version, proto_len) = parse_var_int(reader).await?;

        //size_of::<u8> because the last varInt is one byte in length (FIXME)
        let server_addr_len = len - proto_len - size_of::<UShort>() - size_of::<u8>();
        let server_address = parse_server_addr(reader, server_addr_len).await?;

        let server_port = parse_ushort(reader).await?;

        let (intent_raw, intent_raw_len) = parse_var_int(reader).await?;

        let intent = Intent::try_from(intent_raw).expect("todo: invalid packet handling");

        Ok(Handshake {
            protocol_version,
            server_address,
            server_port,
            intent,
        })
    }


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl TryFrom<i32> for Intent {
    type Error = ();
    fn try_from(val: i32) -> Result<Intent, ()> {
        match val {
            1 => Ok(Intent::Status),
            2 => Ok(Intent::Login),
            3 => Ok(Intent::Transfer),
            _ => Err(()),
        }
    }
}
