use crate::out::Transfer;

use tokio::io;

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
    iter: T,
}

impl<T> Array<T>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: Transfer,
{
    pub fn new(iter: T) -> Array<T> {
        Array { iter }
    }
}

#[async_trait::async_trait]
impl<T> Transfer for Array<T>
where
    T: Transfer,
{
    async fn write_to_tcp_stream(&self, stream: tokio::net::TcpStream) -> io::Result<()> {
        for item in self.iter {
            item.write_to_tcp_stream(stream)?;
        }
        Ok(())
    }
}
