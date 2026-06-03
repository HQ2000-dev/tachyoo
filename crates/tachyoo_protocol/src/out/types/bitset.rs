//maybe just type BitSet = PrefixedArray<Long>??? (TODO)

use tokio::io;

use crate::out::{
    Transfer, Writable,
    types::{Long, array::PrefixedArray},
};

pub struct BitSet {
    inner: PrefixedArray<Long>,
}

impl BitSet {
    pub fn from_iter(iter: impl Iterator<Item = Long>) -> BitSet

    {
        BitSet {
            inner: PrefixedArray::from_iter(iter),
        }
    }

    pub fn new(data: Box<[Long]>) -> BitSet {
        BitSet { inner: PrefixedArray::new(data) }
    }
}

#[async_trait::async_trait]
impl Transfer for BitSet {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.inner.write_data(writeable).await
    }
}


pub struct FixedBitSet {
    
}