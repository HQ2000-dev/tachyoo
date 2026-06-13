pub mod event_in;
pub mod event_out;

use std::convert::Infallible;

use tokio::{net::TcpStream, sync::mpsc};

use crate::{error::ServerError, player_task::{event_in::PlayerInEvent, event_out::PlayerOutEvent}};

enum PlayerEvent {
    TcpStreamData(),
}

pub async fn player_task(conn: TcpStream, event_tx: mpsc::Sender<PlayerOutEvent>, event_rx: mpsc::Receiver<PlayerInEvent>) -> Result<Infallible, ServerError> {
    
    loop {
        
    }
}