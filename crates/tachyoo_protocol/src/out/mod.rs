use std::io::Write;

pub mod packet;
pub mod packet_ids;
pub mod types;
pub mod packets;

//TODO: benchmark write_bytes perf
// TODO: make Transfer types configs again (maybe) and cache the buffer
//types that can be infallibly transmitted
pub trait Transfer {
    //big endian!!!!
    fn write_bytes(&self, buf: &mut Buffer);

    fn size_hint() -> Option<usize> {
        None
    }

    //tmporary helper
    fn data(&self) -> Box<[u8]> {
        let mut vec = Vec::new();

        let mut buf = Buffer::new(vec.as_mut_slice());

        self.write_bytes(&mut buf);

        vec.into_boxed_slice()
    }
}

pub struct Buffer<'a>(&'a mut [u8]);

impl<'a> Buffer<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self(data)
    }

    //TODO: rename to write_all_be?
    pub fn write_all(&mut self, buf: &[u8]) {
        self.0.write_all(buf).unwrap();
    }
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

pub trait TransferablePacket: Transfer {
    const ID: i32;
}