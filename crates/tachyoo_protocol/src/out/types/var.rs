////// INT
mod int {

    use std::ops::Deref;

    // #[repr(transparent)]
    pub struct VarInt {
        buffer: Box<[u8]>,
    }

    pub fn var_int(num: i64) -> VarInt {
        let capacity_approx = 0;
        let mut bytes = Vec::with_capacity(capacity_approx);

        match leb128::write::signed(&mut bytes, num) {
            Ok(_bytes_written) => VarInt {
                buffer: bytes.into_boxed_slice(),
            },
            Err(_) => unreachable!("Vec's Write::write() impl never returns an error"),
        }
    }

    impl AsRef<[u8]> for VarInt {
        fn as_ref(&self) -> &[u8] {
            &self.buffer.deref().as_ref()
        }
    }
}

pub mod long {

    use std::ops::Deref;

    #[repr(transparent)]
    pub struct VarLong {
        buffer: Box<[u8]>,
    }

    impl AsRef<[u8]> for VarLong {
        fn as_ref(&self) -> &[u8] {
            &self.buffer.deref().as_ref()
        }
    }
}

pub mod error {

    #[derive(Debug, thiserror::Error)]
    #[error("the VarInt wouldn't fit into 32 bits")]
    pub struct VarIntError(());

    #[derive(Debug, thiserror::Error)]
    #[error("the VarLong wouldn't fit into 64 bits")]
    pub struct VarLongError(());
}
