pub mod packet_ids;
pub mod types;
pub mod var;

use std::io;

use bytes::{BufMut, BytesMut};
use tokio::io::AsyncReadExt;

use crate::{in_::packet_ids::Packet, state::ProtocolStage};

//TODO: try out making it generic over the stage
pub struct ProtocolParser {
    buffer: BytesMut,
    stage: ProtocolStage,
}

impl ProtocolParser {
    pub fn new() -> ProtocolParser {
        ProtocolParser {
            buffer: BytesMut::new(),
            stage: ProtocolStage::default(),
        }
    }

    pub fn set_stage(&mut self, new_stage: ProtocolStage) {
        self.stage = new_stage;
    }

    pub async fn parse_next<R: AsyncReadExt>(&mut self, reader: &mut R) -> io::Result<Packet> {
        match self.stage {
            ProtocolStage::Handshake => todo!(),
            _ => todo!(),
        }
    }
}
