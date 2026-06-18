pub mod int {
    pub mod signed {
        use std::{
            io::{self, ErrorKind::UnexpectedEof},
            thread,
        };

        use leb128::read::Error;
        use tokio::{io::AsyncReadExt, task::block_in_place};

        use crate::in_::types::Int;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse_var_int<R: AsyncReadExt + Unpin>(
            reader: &mut R,
        ) -> io::Result<(Int, usize)> {
            let mut total = Vec::new();
            for i in 1.. {
                let mut data = [0u8];
                reader.read_exact(&mut data).await?;
                total.push(data[0]);
                match leb128::read::unsigned(&mut total.as_slice()) {
                    Err(e) => match e {
                        Error::IoError(e) if e.kind() == UnexpectedEof => {}
                        Error::IoError(e) => return Err(e),
                        Error::Overflow => todo!(),
                    },

                    Ok(val) => return Ok((val as i32, i)),
                }
            }
            panic!()
        }
    }
    pub mod unsigned {
        use std::io::{self, ErrorKind::UnexpectedEof};

        use leb128::read::Error;
        use tokio::{io::AsyncReadExt, task::block_in_place};

        use crate::in_::types::UInt;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse_var_uint<R: AsyncReadExt + Unpin>(
            reader: &mut R,
        ) -> io::Result<(UInt, usize)> {
            let mut total = Vec::new();
            for i in 1.. {
                let mut data = [0u8];
                reader.read_exact(&mut data).await?;
                total.push(data[0]);
                match leb128::read::unsigned(&mut total.as_slice()) {
                    Err(e) => match e {
                        Error::IoError(e) if e.kind() == UnexpectedEof => {}
                        Error::IoError(e) => return Err(e),
                        Error::Overflow => todo!(),
                    },

                    Ok(val) => return Ok((val as u32, i)),
                }
            }
            panic!()
        }
    }
}

pub mod long {
    pub mod signed {
        use std::io::{self, ErrorKind::UnexpectedEof};

        use leb128::read::Error;
        use tokio::{io::AsyncReadExt, task::block_in_place};

        use crate::in_::types::{Long, UInt};

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse_var_long<R: AsyncReadExt + Unpin>(
            reader: &mut R,
        ) -> io::Result<(Long, usize)> {
            let mut total = Vec::new();
            for i in 1.. {
                let mut data = [0u8];
                reader.read_exact(&mut data).await?;
                total.push(data[0]);
                match leb128::read::signed(&mut total.as_slice()) {
                    Err(e) => match e {
                        Error::IoError(e) if e.kind() == UnexpectedEof => {}
                        Error::IoError(e) => return Err(e),
                        Error::Overflow => todo!(),
                    },

                    Ok(val) => return Ok((val, i)),
                }
            }
            panic!()
        }
    }
    pub mod unsigned {
        use std::io::{self, ErrorKind::UnexpectedEof};

        use leb128::read::Error;
        use tokio::{io::AsyncReadExt, task::block_in_place};

        use crate::in_::types::ULong;

        //TODO: own implementaion!!!
        // (this is very inefficient, I think)
        pub async fn parse_var_ulong<R: AsyncReadExt + Unpin>(
            reader: &mut R,
        ) -> io::Result<(ULong, usize)> {
            let mut total = Vec::new();
            for i in 1.. {
                let mut data = [0u8];
                reader.read_exact(&mut data).await?;
                total.push(data[0]);
                match leb128::read::unsigned(&mut total.as_slice()) {
                    Err(e) => match e {
                        Error::IoError(e) if e.kind() == UnexpectedEof => {}
                        Error::IoError(e) => return Err(e),
                        Error::Overflow => todo!(),
                    },

                    Ok(val) => return Ok((val, i)),
                }
            }
            panic!()
        }
    }
}
