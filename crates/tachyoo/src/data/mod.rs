use crate::data::{client::ClientData, connection::ConnectionState};

pub mod client;
pub mod connection;

pub struct Data {
    pub client: ClientData,
    pub conn: ConnectionState,
}

impl Data {
    pub fn new() -> Data {
        Data {
            client: ClientData::new(),
            conn: ConnectionState::new(),
        }
    }
}
