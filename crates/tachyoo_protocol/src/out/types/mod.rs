// everything is big endian!!

pub enum Boolean {
    False = 0x00,
    True = 0x01,
}

pub type Byte = i8;

pub type UByte = u8;
pub type Short = i16;
pub type UShort = u16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
//TODO: length encoding
//it's actually String(n), maybe partially encode that?
pub struct String {}

pub struct VarInt {}

pub struct VarLong {}
