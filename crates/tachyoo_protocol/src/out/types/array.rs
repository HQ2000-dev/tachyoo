use crate::out::{Transfer, Writable, types::var::int::VarInt};

use tokio::io::{self, AsyncWriteExt};

/*
impl<T, I> IntoTransferable for T where T: Iterator<Item=I>, I: Transfer {
    type Transferable = Array<T>;
    type Error = Infallible;

    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
        Ok(Array(self))
    }
}
*/

//TODO: opt: dense array write (if Transfer gets removed)
pub struct Array<T> {
    data: Box<[T]>,
}

impl<T> Array<T>
where
    T: Transfer,
{
    pub fn new(data: Box<[T]>) -> Array<T> {
        Array { data }
    }

    pub fn from_iter(iter: impl Iterator) -> Array<T> {
        //TODO: adjust cap estimate
        let est_capacity = iter.size_hint().0;
        let data = Vec::with_capacity(est_capacity).into_boxed_slice();

        Array { data }
    }
}

#[async_trait::async_trait]
impl<T> Transfer for Array<T>
where
    T: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        for item in &self.data {
            item.write_data(writeable).await?;
        }
        Ok(())
    }
}

pub struct PrefixedArray<T> {
    data: Box<[T]>,
}

impl<T> PrefixedArray<T>
where
    T: Transfer,
{
    pub fn new(data: Box<[T]>) -> PrefixedArray<T> {
        PrefixedArray { data }
    }

    pub fn from_iter(iter: impl Iterator<Item = T>) -> PrefixedArray<T> {
        let est_capacity = iter.size_hint().1.unwrap_or(5);
        let data = Vec::with_capacity(est_capacity).into_boxed_slice();

        PrefixedArray { data }
    }
}

#[async_trait::async_trait]
impl<T> Transfer for PrefixedArray<T>
where
    T: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        VarInt::new(self.data.len() as i32)
            .write_data(writeable)
            .await?;

        for item in &self.data {
            item.write_data(writeable).await?;
        }

        //TODO: debug_assert!(count == expected)
        Ok(())
    }
}
