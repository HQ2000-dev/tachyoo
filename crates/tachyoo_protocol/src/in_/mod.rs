pub mod state;
pub mod types;
pub mod var;

use bytes::{BufMut, BytesMut};
use tokio::io::{self, AsyncReadExt};

pub struct ProtocolParser {
    buffer: BytesMut,
}

impl ProtocolParser {
    /*pub async fn parse_next<R: AsyncReadExt + Unpin>(&mut self, r: &mut R) -> io::Result<Option> {
        // r.read(buf)

        Ok(())
    }*/
}
