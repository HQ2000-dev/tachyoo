use std::io;

use tokio::io::AsyncReadExt;

use crate::in_::types::Long;
use crate::in_::{packets::Packet, types::parse_long};

#[derive(Debug, Clone)]
pub struct PingRequest {
    timestamp: Long,
}

pub async fn parse_ping_request<R: AsyncReadExt + Unpin>(
    reader: &mut R,
) -> io::Result<PingRequest> {
    println!("waiting for a long");
    let timestamp = parse_long(reader).await?;
    println!("got a long");
    Ok(PingRequest { timestamp })
}
