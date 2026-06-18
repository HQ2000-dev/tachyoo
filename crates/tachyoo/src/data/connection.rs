use tachyoo_protocol::in_::packets::Compression;

pub struct ConnectionState {
    compression: Compression,
    inner: Stat
}

#[derive(Default, Debug)]
pub enum ProtocolStateInner {
    #[default]
    Handshake {
        
    },
    Status,
    Login,
    Config,
    Play,
}