use tachyoo_protocol::in_::{
    packet::Compression,
    types::handshake::{Handshake, Intent},
};
use tachyoo_protocol::stage::ProtocolStage;

pub struct ConnectionState {
    pub compression: Compression,
    pub stage: ProtocolStage,
}

impl ConnectionState {
    pub fn new() -> ConnectionState {
        ConnectionState {
            compression: Compression::default(),
            stage: ProtocolStage::default(),
        }
    }

    //TODO: maybe a newtype? and just register player after logn?
    pub fn handshake_complete(&mut self, handshake: &Handshake) {
        assert_eq!(self.stage, ProtocolStage::Handshake);
        match handshake.intent {
            Intent::Status => self.stage = ProtocolStage::Status,
            Intent::Login => self.stage = ProtocolStage::Login,
            Intent::Transfer => todo!("implement transfer handling"),
        }
    }
}
