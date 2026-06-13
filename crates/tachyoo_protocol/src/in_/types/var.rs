pub mod int {
    pub mod signed {
    use std::io;

    use leb128::read::Error;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::SyncIoBridge;

use crate::in_::types::Int;

    //TODO: own implementaion!!!
    // (this is very inefficient, I think)
    pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Int> {
        let mut bridge = SyncIoBridge::new(reader);
        leb128::read::signed(&mut bridge)
            .map_err(|e| match e {
                Error::IoError(err) => err,
                Error::Overflow => todo!(),
            })
            .map(|val| val as i32)
    }
    }
    pub mod unsigned {
    use std::io;

    use leb128::read::Error;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::SyncIoBridge;

use crate::in_::types::UInt;

    //TODO: own implementaion!!!
    // (this is very inefficient, I think)
    pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<UInt> {
        let mut bridge = SyncIoBridge::new(reader);
        leb128::read::unsigned(&mut bridge)
            .map_err(|e| match e {
                Error::IoError(err) => err,
                Error::Overflow => todo!(),
            })
            .map(|val| val as u32)
    }
    }
}

pub mod long {
    use std::io;

    use leb128::read::Error;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::SyncIoBridge;

use crate::in_::types::Long;

    //TODO: own implementaion!!!
    // (this is very inefficient, I think)
    pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Long> {
        let mut bridge = SyncIoBridge::new(reader);
        leb128::read::signed(&mut bridge)
            .map_err(|e| match e {
                Error::IoError(err) => err,
                Error::Overflow => todo!(),
            })
    }
    pub mod unsigned {
    use std::io;

    use leb128::read::Error;
    use tokio::io::AsyncReadExt;
    use tokio_util::io::SyncIoBridge;

use crate::in_::types::ULong;

    //TODO: own implementaion!!!
    // (this is very inefficient, I think)
    pub async fn parse<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<ULong> {
        let mut bridge = SyncIoBridge::new(reader);
        leb128::read::unsigned(&mut bridge)
            .map_err(|e| match e {
                Error::IoError(err) => err,
                Error::Overflow => todo!(),
            })
    }
    }
}
