use tokio::io::AsyncWriteExt;

//maybe still a non-raw version for compresion handling? or just bytes?
//raw presentation of the data
pub enum Packet {
    Uncompressed { packet_id: u8, bytes: bytes::Bytes },
    Compressed {},
}

impl Packet {
    //pub fn from_

    pub fn actual_len(&self) -> usize {
        //self.bytes.len() + 1 // self.packet_id
        todo!()
    }

    pub fn send(&self, stream: tokio::net::TcpStream) {}
}
