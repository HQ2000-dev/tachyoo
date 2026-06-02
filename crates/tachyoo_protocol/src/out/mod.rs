use tokio::io::{self, AsyncWriteExt};

pub(super) mod raw_packet;
pub mod types;

//trait for types that implement (fallible) conversion into transferable types
pub trait IntoTransferable {
    type Transferable: Transfer;
    type Error: std::error::Error;
    /* async */
    fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error>;
}

//TODO: define cancel safety requirements
//types that are reprs for transmission
#[async_trait::async_trait] //TODO: tmp
pub trait Transfer {
    async fn write_to_tcp_stream(&self, stream: tokio::net::TcpStream) -> Result<(), io::Error>;

    /*    async fn write_to_tcp_stream(
        &self,
        mut stream: tokio::net::TcpStream,
    ) -> Result<(), io::Error> {
        //for cancel safety (FIXME)
        stream.write_all_buf(&mut self.as_bytes()).await?;
        Ok(())
    }*/
}

/*
macro_rules! impl_transfer {
($N:expr, $($T:ident),*) => {
        impl<$($T),*> Transfer for ($($T,)*)
       where T: Transfer,
        {
           // type Tup = ($(Foo<$T>,)*);

            fn write_to_tcp_stream(&self, mut stream: tokio::net::TcpStream) -> Result< {

                tokio::try_join!(
                    stream.write_all_buf(&mut self.$N.as_ref()),

                )
            }
        }
    };
}

variadics_please::all_tuples!(impl_transfer, 1, 15, T);
*/
