pub mod packets;
pub mod types;
pub mod var;

use std::io::{self, Read};

use bytes::{BufMut, BytesMut};
use tokio::{io::AsyncReadExt, task::block_in_place};

use crate::{
    in_::{
        packets::{Compression, Packet},
        types::{
            handshake::parse_handshake,
            var::int::{signed::parse_var_int, unsigned::parse_var_uint},
        },
    },
    state::ProtocolStage,
};

//TODO: try out making it generic over the stage
#[derive(Debug)]
pub struct ProtocolParser {
    buffer: BytesMut,
    stage: ProtocolStage,
    compression: Compression,
}

impl ProtocolParser {
    pub fn new() -> ProtocolParser {
        ProtocolParser {
            buffer: BytesMut::new(),
            stage: ProtocolStage::default(),
            compression: Compression::default(),
        }
    }

    pub fn set_stage(&mut self, new_stage: ProtocolStage) {
        self.stage = new_stage;
    }

    pub async fn parse_packet<R: AsyncReadExt + Unpin>(
        &mut self,
        reader: &mut R,
    ) -> io::Result<Packet> {
        let (packet_len, len_of_packet_len) = parse_var_int(reader).await?;

        if packet_len < 0 {
            todo!("handle negative packet sizes (should not happen)");
        }

        let (id, id_len) = parse_var_int(reader).await?;

        if let Compression::Compressed { threshold } = self.compression {
            let (data_len, _) = parse_var_int(reader).await?;
            if data_len < 0 {
                todo!("handle negative len (invalid packet)");
            }

            /*let mut compressed = Vec::with_capacity(data_len as usize);
            reader.read_to_end(&mut compressed).await?;
            let decompressed = block_in_place(|| {
                let mut uncompressed: Vec<u8> = Vec::with_capacity(data_len as usize);
                flate2::read::ZlibDecoder::new(compressed.as_slice()).read_to_end(&mut uncompressed)?;
                Ok::<_, io::Error>(uncompressed)
            })?;
            parse_packet_inn*/
            todo!("compressed packets")
        } else {
            //no truncating cast, nonnegative
            self.parse_packet_inner(reader, packet_len as usize - id_len, id)
                .await
        }
    }

    pub async fn parse_packet_inner<R: AsyncReadExt + Unpin>(
        &mut self,
        reader: &mut R,
        len: usize,
        id: i32,
    ) -> io::Result<Packet> {
        Ok(match self.stage {
            ProtocolStage::Handshake => match id {
                0 => Packet::Handshake(parse_handshake(reader, len).await?),
                id @ i32::MIN.. => todo!("invalid packet id for handshake: {id}"),
            },
            ProtocolStage::Status => match id {
                id @ i32::MIN.. => todo!("invalid packet id for status: {id}"),
            },
            ProtocolStage::Login => todo!(),
            ProtocolStage::Config => todo!(),
            ProtocolStage::Play => todo!(),
        })
    }
}
