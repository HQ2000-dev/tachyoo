use tokio::io;

use crate::out::{IntoTransferable, Transfer};

#[derive()]
pub struct Optional<T>(Option<T>);

impl<T> IntoTransferable for Option<T>
where
    T: IntoTransferable,
    <T as IntoTransferable>::Transferable: Send + Sync,
{
    type Transferable = Optional<<T as IntoTransferable>::Transferable>;
    type Error = <T as IntoTransferable>::Error;

    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
        Ok(Optional(
            self.map(|t| t.try_into_transferable()).transpose()?,
        ))
    }
}

#[async_trait::async_trait]
impl<T> Transfer for Optional<T>
where
    T: Transfer + Send + Sync,
{
    async fn write_to_tcp_stream(&self, stream: tokio::net::TcpStream) -> Result<(), io::Error> {
        match self.0 {
            Some(ref t) => t.write_to_tcp_stream(stream).await,
            None => Ok(()),
        }
    }
}
