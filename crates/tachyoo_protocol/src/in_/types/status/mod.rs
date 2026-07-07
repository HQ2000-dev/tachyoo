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
    let timestamp = parse_long(reader).await?;
    Ok(PingRequest { timestamp })
}
