use tokio::io::AsyncWriteExt;

//max len:
pub enum RawPacket {
    Uncompressed { packet_id: u8, bytes: bytes::Bytes },
    Compressed {},
}

impl RawPacket {
    //pub fn from_

    pub fn len(&self) -> usize {
        self.bytes.len() + 1 // self.packet_id
    }

    pub fn send(&self, stream: tokio::net::TcpStream) {}
}
