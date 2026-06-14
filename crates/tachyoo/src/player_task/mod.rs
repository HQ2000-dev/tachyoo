pub mod event_in;
pub mod event_out;

use std::convert::Infallible;

use tachyoo_protocol::in_::{ProtocolParser, packet_ids::Packet as InPacket};
use tachyoo_protocol::out::packet::Packet as OutPacket;
use tokio::runtime::Handle;
use tokio::{
    net::TcpStream,
    select, spawn,
    sync::{broadcast, mpsc, watch},
};
use tokio_util::sync::CancellationToken;

use crate::{
    ShutdownMsg,
    error::ServerError,
    player_task::{event_in::PlayerInEvent, event_out::PlayerOutEvent},
    util::cancel_able,
};

enum PlayerEvent {
    ReceivedPacket(Packet),
    ReceivedEvent(PlayerInEvent),
}

pub async fn player_task(
    handle: Handle,
    cancel_token: CancellationToken,
    conn: TcpStream,
    event_tx: mpsc::Sender<PlayerOutEvent>,
    event_rx: mpsc::Receiver<PlayerInEvent>,
) -> Result<(), ServerError> {

    //(TODO: determine perf implications)
    let (conn_read, conn_write) = conn.into_split();
    
    //TODO: determine limit
    let (tx, rx) = mpsc::channel(999);

    //TODO: cancel safety requirements??
    let recv_task = spawn(cancel_able(cancel_token.child_token(), async {
        loop {
            tx.send(PlayerEvent::ReceivedEvent(event_rx.recv().await.unwrap()))
                .await
                .unwrap()
        }
    }));

    let parser = ProtocolParser::new();

    //same here
    let packet_task = spawn(cancel_able(cancel_token.child_token(), async {
        loop {
            tx.send(PlayerEvent::ReceivedPacket(
                parser.parse_next(&mut conn_read).await?,
            ))
            .await
            .unwrap()
        }
    }))?;

    let (write_tx, write_rx) = mpsc::channel(999);

    //maybe blocking on compression via a rt handle??
    let write_task=spawn(cancel_able(cancel_token.child_token(), async {
        loop {
            write_rx.recv().await.unwrap()
        }
    }))?;

    cancel_able(cancel_token, async { loop {
        
    } }).await
}
