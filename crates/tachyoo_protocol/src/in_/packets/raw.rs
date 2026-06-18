use std::io::{self, Read};

use tokio::{io::AsyncReadExt, task::block_in_place};

use crate::in_::{
    packets::{Compression, Packet},
    types::var::int::signed::parse_var_int,
};

pub struct RawPacket<'a, R: AsyncReadExt + Unpin> {
    len: i32,
    id: i32,
    reader: &'a mut R,
}
