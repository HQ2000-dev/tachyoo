use tokio::io;

use crate::out::{Transfer, Writable};

pub struct Pos {
    data: [u8; 8],
}

impl Pos {
    //TODO: verify that these are right
    pub const MIN_X: i32 = -16_777_216;
    pub const MAX_X: i32 = 16_777_215;
    pub const MIN_Z: i32 = -16_777_216;
    pub const MAX_Z: i32 = 16_777_215;
    pub const MIN_Y: i16 = -1024;
    pub const MAX_Y: i16 = 1023;

    //TODO: more elaborate error maybe?
    pub fn new_xyz(x: i32, y: i16, z: i32) -> Option<Pos> {
        if Pos::in_bounds(x, y, z) {
            Some(Pos::new_unchecked(x as i64, y as i64, z as i64))
        } else {
            None
        }
    }

    //needs to stay private!!!
    fn new_unchecked(x: i64, y: i64, z: i64) -> Pos {
        //straight copied from minecraft.wiki
        let data: i64=
            //straight copied from minecraft.wiki
            ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF);

        Pos {
            data: data.to_be_bytes(),
        }
    }

    fn in_bounds(x: i32, y: i16, z: i32) -> bool {
        x >= Self::MIN_X
            && x <= Self::MAX_X
            && y >= Self::MIN_Y
            && y <= Self::MAX_Y
            && z >= Self::MIN_Z
            && z <= Self::MAX_Z
    }
}

#[async_trait::async_trait]
impl Transfer for Pos {
    async fn write_data(&self, writer: &mut Writable) -> io::Result<()> {
        writer.write_all(&self.data).await
    }
}
