////// INT
pub mod int {

    use std::convert::Infallible;

    use tokio::io;

    use tokio::io::AsyncWriteExt;

    use crate::out::{IntoTransferable, Transfer};

    // #[repr(transparent)]
    pub struct VarInt {
        data: Box<[u8]>,
    }

    impl VarInt {
        pub fn new(num: i32) -> VarInt {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, num as i64) {
                Ok(_bytes_written) => VarInt {
                    data: bytes.into_boxed_slice(),
                },
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }

    /*impl IntoTransferable for i32 {
        type Transferable = VarInt;
        type Error = Infallible;
        fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, self as i64) {
                Ok(_bytes_written) => Ok(VarInt {
                    data: bytes.into_boxed_slice(),
                }),
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }*/

    #[async_trait::async_trait]
    impl Transfer for VarInt {
        async fn write_to_tcp_stream(
            &self,
            mut stream: tokio::net::TcpStream,
        ) -> Result<(), io::Error> {
            stream.write_all(&self.data).await
        }
    }
}

pub mod long {
    use tokio::io;

    use std::convert::Infallible;

    use tokio::io::AsyncWriteExt;

    use crate::out::Transfer;

    /*impl IntoTransferable for i64 {
        type Transferable = VarLong;
        type Error = Infallible;
        fn try_into_transferable(self) -> Result<Self::Transferable, Self::Error> {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, self) {
                Ok(_bytes_written) => Ok(VarLong {
                    data: bytes.into_boxed_slice(),
                }),
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }*/

    #[repr(transparent)]
    pub struct VarLong {
        data: Box<[u8]>,
    }

    impl VarLong {
        pub fn new(num: i64) -> VarLong {
            let capacity_approx = 0;
            let mut bytes = Vec::with_capacity(capacity_approx);

            match leb128::write::signed(&mut bytes, num) {
                Ok(_bytes_written) => VarLong {
                    data: bytes.into_boxed_slice(),
                },
                Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
            }
        }
    }

    #[async_trait::async_trait]
    impl Transfer for VarLong {
        async fn write_to_tcp_stream(
            &self,
            mut stream: tokio::net::TcpStream,
        ) -> Result<(), io::Error> {
            stream.write_all(&self.data).await
        }
    }
}
