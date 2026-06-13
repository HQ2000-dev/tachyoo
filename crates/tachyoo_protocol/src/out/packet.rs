use std::{io::Write, marker::PhantomData};

use flate2::{Compress, write::ZlibEncoder};
use tokio::io::AsyncWriteExt;

use crate::out::{Transfer, types::var::int::VarInt};

//having the binary data as an input? (TODO)
//maybe still a non-raw version for compresion handling? or just bytes?
//raw presentation of the data
pub enum Packet<T> {
    Uncompressed {
        len: VarInt,
        id: VarInt,
        //TODO: tmp
        data: Box<[u8]>,
        phantom: PhantomData<T>,
    },
    Compressed {
        len: VarInt,
        //TODO: tmp
        data: Box<[u8]>,
        phantom: PhantomData<(T, VarInt)>,
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

impl<T: Transfer> Packet<T> {
    pub async fn new(id: i32, transfer: T, compression: Compression) -> Packet<T>
    where
        T: Transfer,
    {
        let mut data = transfer.data();

        if let Compression::Compressed { threshold, level } = compression {
            //tmp
            if data.len() as i32 >= threshold {
                data = compress(&data, level);
            }

            Packet::Compressed {
                len: VarInt::new(data.len() as i32),
                data,
                phantom: PhantomData,
            }
        } else {
            Packet::Uncompressed {
                len: VarInt::new(data.len() as i32),
                id: VarInt::new(id),
                data,
                phantom: PhantomData,
            }
        }
    }

    pub async fn new_uncompressed(id: i32, transfer: T) -> Packet<T> {
        Packet::new(id, transfer, Compression::Uncompressed).await
    }

    pub async fn send(&self, mut stream: tokio::net::TcpStream) -> tokio::io::Result<()> {
        match self {
            Packet::Compressed { len, data, phantom } => {
                stream.write_all(&len.data()).await?;
                stream.write_all(&data).await
            }
            Packet::Uncompressed {
                len,
                id,
                data,
                phantom,
            } => {
                stream.write_all(&len.data()).await?;
                stream.write_all(&data).await
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
