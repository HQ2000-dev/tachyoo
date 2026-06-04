use crate::out::types::var::int::VarInt;

//better name!
pub struct McString {
    //max: 32767
    len: VarInt,
    data: Box<str>,
}

impl McString {
    pub const MAX_LENGTH: u16 = 32767;

    pub fn len(&self) -> usize {}
}

//TODO: maybe impl Into<Box<str>>??
