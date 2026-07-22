////// INT
pub mod int {

    use crate::out::{Buffer, Transfer};

    #[derive(Debug)]
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

    impl Transfer for VarInt {
        fn write_bytes(&self, buf: &mut Buffer) {
            buf.write_all(&self.data);
        }
    }
}

pub mod long {

    use crate::out::{Buffer, Transfer};

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

    impl Transfer for VarLong {
        fn write_bytes(&self, buf: &mut crate::out::Buffer) {
            buf.write_all(&self.data);
        }
    }
}
