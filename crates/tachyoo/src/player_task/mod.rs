pub mod event_in;
pub mod event_out;

use std::convert::Infallible;

use tachyoo_protocol::in_::{ProtocolParser, packet_ids::Packet as InPacket};
use tachyoo_protocol::out::packet::Packet as OutPacket;
use tokio::io::AsyncReadExt;
use tokio::runtime::Handle;
use tokio::task::JoinSet;
use tokio::{
    net::TcpStream,
    select, spawn,
    sync::{broadcast, mpsc, watch},
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    ShutdownMsg,
    error::ServerError,
    player_task::{event_in::PlayerInEvent, event_out::PlayerOutEvent},
    util::cancel_able,
};

enum PlayerEvent {
    //TODO
    ReceivedPacket(()),
    ReceivedEvent(PlayerInEvent),
    #[cfg(feature = "dev")]
    ReceivedData(Vec<u8>),
}

//TODO: use try_read/write to detect if the stream was closed
//TODO: self-cancelling
pub async fn player_task(
    handle: Handle,
    cancel_token: CancellationToken,
    conn: TcpStream,
    event_tx: mpsc::Sender<PlayerOutEvent>,
    mut event_rx: mpsc::Receiver<PlayerInEvent>,
) -> Result<(), ServerError> {

    eprintln!("entered player task");

    let mut local_join_set=JoinSet::<Result<(), ServerError>>::new();

    //(TODO: determine perf implications)
    let (mut conn_read, mut conn_write) = conn.into_split();
    
    //TODO: determine limit
    let (msg_tx, msg_rx) = mpsc::channel::<PlayerEvent>(999);

    //TODO: just require PlayerEvents to be sent?
    // tmp commented out
    /*let msg_recv_task = spawn(cancel_able::<_, Infallible>(cancel_token.child_token(), async move {
        loop {
            msg_tx.send(PlayerEvent::ReceivedEvent(event_rx.recv().await.unwrap()))
                .await
                .unwrap()
        }
    })).await.unwrap();*/

    let (packet_read_tx, mut packet_read_rx) = mpsc::channel(999);

    let parser = ProtocolParser::new();

    //same here
    local_join_set.spawn(cancel_able(cancel_token.child_token(), async move {
        eprintln!("prepared reading packets");
        loop {
            packet_read_tx.send(
                //  PlayerEvent::ReceivedPacket(
                    //    parser.parse_next(&mut conn_read).await.expect("TODO: proper io error (especially unexpected eof) handling!"),
            //)
            PlayerEvent::ReceivedData({
                let mut buf=Vec::new();
                conn_read.read_buf(&mut buf).await.unwrap();
                buf
            })
            )
            .await
            .unwrap();
            //eprintln!("sent an msg");
        }
    }));
    //.await.unwrap()?;

    let (packet_write_tx, mut packet_write_rx) = mpsc::channel(999);

    //maybe blocking on compression via a rt handle??
    local_join_set.spawn(cancel_able(cancel_token.child_token(), async move {
        loop {
            packet_write_rx.recv().await.unwrap()
        }
    }));



    local_join_set.spawn(cancel_able(cancel_token, async move { 
        eprintln!("started main player loop");
        loop {
        
            let msg = packet_read_rx.recv().await.unwrap();
                if let PlayerEvent::ReceivedData(data) = msg {
                    if !data.is_empty() {
                        eprintln!("{:?}", data);
                    } else {
                        
                    }
                } else {
                    unreachable!("well...");
                }
            
        
    } }));

    for result in local_join_set.join_next().await.unwrap() {
        result?;
    }
    Ok(())
}
