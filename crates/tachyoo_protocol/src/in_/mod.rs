pub mod packets;
pub mod types;
pub mod var;

use std::io::{self, Read};

use bytes::{BufMut, BytesMut};
use tokio::{io::AsyncReadExt, task::block_in_place};

use crate::in_::{
    packets::{Compression, Login, Packet, Status},
    types::{
        handshake::{Intent, parse_handshake},
        status::parse_ping_request,
        var::int::{signed::parse_var_int, unsigned::parse_var_uint},
    },
};
use crate::stage::ProtocolStage;

//TODO: try out making it generic over the stage
#[derive(Debug)]
pub struct ProtocolParser {
    //probably unused
    buffer: BytesMut,
    stage: ProtocolStage,
    compression: Compression,
}

impl ProtocolParser {
    pub fn new() -> ProtocolParser {
        ProtocolParser {
            buffer: BytesMut::new(),
            compression: Compression::default(),
            stage: ProtocolStage::default(),
        }
    }

    pub fn stage(&self) -> &ProtocolStage {
        &self.stage
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

        //tokio::io::copy!!!
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
                0 => {
                    let handshake = parse_handshake(reader, len).await?;
                    self.stage = match handshake.intent {
                        Intent::Status => ProtocolStage::Status,
                        Intent::Login | Intent::Transfer => ProtocolStage::Login,
                    };
                    Packet::Handshake(handshake)
                }
                id @ i32::MIN.. => todo!("invalid packet id for handshake: {id}"),
            },
            ProtocolStage::Status => match id {
                0 => Packet::Status(Status::StatusRequest),
                1 => Packet::Status(Status::PingRequest(parse_ping_request(reader).await?)),
                id @ i32::MIN.. => todo!("invalid packet id for status: {id}"),
            },
            ProtocolStage::Login => match id {
                0 => Packet::Login(),
            },
            ProtocolStage::Config => todo!(),
            ProtocolStage::Play => todo!(),
        })
    }
}

/*
pub trait Parseable: Sized {
    //TODO: associated type defaults
    type Err;

    async fn parse<R: AsyncReadExt + Unpin>(
        reader: &mut R,
        len: usize,
    ) -> Result<Self, <Self as Parseable>::Err>;
}
*/