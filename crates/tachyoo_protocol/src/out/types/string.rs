use crate::out::types::var::int::VarInt;

//better name!
pub struct String_ {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

impl String_ {
    const MAX_LENGTH: usize = 32767;
}

//TODO: maybe impl Into<Box<str>>??
