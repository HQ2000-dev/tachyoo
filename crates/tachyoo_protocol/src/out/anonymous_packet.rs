use std::{io::Write, marker::PhantomData};

use flate2::{Compress, write::ZlibEncoder};
use tokio::io::AsyncWriteExt;

use crate::out::{Transfer, TransferablePacket, types::var::int::VarInt};

//having the binary data as an input? (TODO)
//maybe still a non-raw version for compression handling? or just bytes?
//raw presentation of the data
pub enum AnonymousPacket {
    Uncompressed {
        len: VarInt,
        id: VarInt,
        //TODO: tmp
        data: Box<[u8]>,
        //_phantom: PhantomData<T>,
    },
    Compressed {
        len: VarInt,
        //TODO: tmp
        data: Box<[u8]>,
        //_phantom: PhantomData<(T, VarInt)>,
    },
}

pub enum Compression {
    Uncompressed,
    Compressed {
        //non-negative!!
        // (negative would mean uncompressed)
        threshold: i32,
        level: flate2::Compression,
    },
}

impl Compression {}

/*
pub struct Compressed<T> {
    data: Box<[u8]>,
    phantom_data: PhantomData<T>,
}

impl<T> Compressed<T> where T: Transfer {
    pub fn new(data: T, level: flate2::Compression) -> Compressed {

    }
}*/

impl AnonymousPacket {
    pub fn with_compression<T: TransferablePacket>(
        transfer: T,
        compression: Compression,
    ) -> AnonymousPacket {
        let mut data = transfer.data();

        if let Compression::Compressed { threshold, level } = compression {
            //tmp (TODO: compress beforehand)
            if data.len() as i32 >= threshold {
                data = tokio::task::block_in_place(|| compress(&data, level));
            }

            AnonymousPacket::Compressed {
                len: VarInt::new(data.len() as i32),
                data,
                //_phantom: PhantomData,
            }
        } else {
            AnonymousPacket::Uncompressed {
                len: VarInt::new(data.len() as i32),
                id: VarInt::new(<T as TransferablePacket>::ID),
                data,
                //_phantom: PhantomData,
            }
        }
    }

    pub fn new<T: TransferablePacket>(transfer: T) -> AnonymousPacket {
        AnonymousPacket::with_compression(transfer, Compression::Uncompressed)
    }

    pub async fn send<R: AsyncWriteExt + Unpin>(
        &self,
        mut writer: &mut R,
    ) -> tokio::io::Result<()> {
        match self {
            AnonymousPacket::Compressed {
                len,
                data,
                //_phantom,
            } => {
                writer.write_all(&len.data()).await?;
                writer.write_all(&data).await
            }
            AnonymousPacket::Uncompressed {
                len,
                id,
                data,
                //_phantom,
            } => {
                writer.write_all(&len.data()).await?;
                writer.write_all(&data).await
            }
        }
    }
}

//tmp!!!
fn compress(data: &[u8], level: flate2::Compression) -> Box<[u8]> {
    let mut vec = Vec::new();
    let mut compressor = ZlibEncoder::new(&mut vec, level);
    compressor.write_all(&data).unwrap();
    drop(compressor);
    vec.into_boxed_slice()
}
