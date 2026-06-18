use crate::data::{client::ClientData, connection::ConnectionState};

pub mod client;
pub mod connection;

pub struct Data {
    client: ClientData,
    conn: ConnectionState,
}