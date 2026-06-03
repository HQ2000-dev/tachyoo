// everything is big endian!!
// (TODO)

use std::ops::Deref;

use tokio::io;

use crate::out::{Transfer, Writable};

pub mod array;
pub mod bitset;
pub mod entity_metadata;
pub mod identifier;
pub mod option;
pub mod pos;
pub mod string;
pub mod var;

pub type Boolean = bool;

#[async_trait::async_trait]
impl Transfer for Boolean {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&[*self as u8]).await
    }
}

pub type Byte = i8;

#[async_trait::async_trait]
impl Transfer for Byte {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&[*self as u8]).await
    }
}

pub type UByte = u8;

#[async_trait::async_trait]
impl Transfer for UByte {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&[*self]).await
    }
}

pub type Short = i16;

#[async_trait::async_trait]
impl Transfer for Short {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type UShort = u16;

#[async_trait::async_trait]
impl Transfer for UShort {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type Int = i32;

#[async_trait::async_trait]
impl Transfer for Int {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type Long = i64;

#[async_trait::async_trait]
impl Transfer for Long {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type Float = f32;

#[async_trait::async_trait]
impl Transfer for Float {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type Double = f64;

#[async_trait::async_trait]
impl Transfer for Double {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}

pub type UUID = u128;


#[async_trait::async_trait]
impl Transfer for UUID {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        writeable.write_all(&self.to_be_bytes()).await
    }
}
