pub mod read {
    pub mod signed {
        use std::io::{self, ErrorKind};

        use leb128::read::Error::{IoError, Overflow};
        use tokio::io::AsyncReadExt;

        pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<i64> {
            let mut buf = [0u8; 10];

            let mut size = 1;
            let decoded = loop {
                reader.read_exact(&mut buf[(size - 1)..size]).await?;

                match leb128::read::signed(&mut &buf[..size]) {
                    Err(IoError(e)) if e.kind() == ErrorKind::UnexpectedEof => size += 1,
                    Err(IoError(e)) => return Err(e),
                    Ok(decoded_) => break decoded_,
                    Err(Overflow) => panic!(),
                }
            };

            Ok(decoded)
        }
    }

    pub mod unsigned {}
}
