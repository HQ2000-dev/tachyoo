use tachyoo_protocol::in_::{
    packets::Compression,
    types::handshake::{Handshake, Intent},
};

pub struct ConnectionState {
    pub compression: Compression,
    pub state: ProtocolState,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        ConnectionState {
            compression: Compression::default(),
            state: ProtocolState::default(),
        }
    }

    //TODO: maybe a newtype? and just register player after logn?
    pub fn handshake_complete(&mut self, handshake: &Handshake) {
        assert!(self.state, ProtocolState::Handshake);
        match handshake.intent {
            Intent::Status => self.state = ProtocolState::Status,
            Intent::Login => self.state = ProtocolState::Login,
            Intent::Transfer => todo!("implement transfer handling"),
        }
    }
}

#[derive(Default, Debug)]
pub enum ProtocolState {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play,
}
