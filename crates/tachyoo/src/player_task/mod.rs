pub mod event_in;
pub mod event_out;

use crate::error::*;

use snafu::ResultExt;
use tachyoo_protocol::{
    in_::{
        ProtocolParser,
        packet::{self as in_packet, Packet as InPacket},
    },
    out::{
        TransferablePacket,
        anonymous_packet::AnonymousPacket,
        packet::{
            self as out_packet, Packet as OutPacket,
            status::{PongResponse, StatusResponse},
        },
    },
    stage::ProtocolStage,
};
use tokio::{
    net::TcpStream,
    runtime::Handle,
    sync::mpsc,
    task::{self, JoinSet, block_in_place, spawn_blocking},
};
use tokio_util::sync::CancellationToken;

use crate::data::Data;
use crate::util::DebugReader;
use crate::{
    error::ServerError,
    player_task::{event_in::PlayerInEvent, event_out::PlayerOutEvent},
    util::cancel_able,
};

#[derive(Debug)]
pub enum PlayerEvent {
    Packet {
        packet: tachyoo_protocol::in_::packet::Packet,
        next_stage: ProtocolStage,
    },
    Event(PlayerInEvent),
    #[cfg(feature = "dev")]
    Data(Vec<u8>),
}

//tmp?
#[derive(Clone, Debug)]
pub struct ProtoStageMsg {
    state: ProtocolStage,
}

//TODO: use try_read/write to detect if the stream was closed
//TODO: self-cancelling
// TODO: separate task for status/login/play?
pub async fn player_task(
    handle: Handle,
    cancel_token: CancellationToken,
    conn: TcpStream,
    //TODO: maybe collapse this into a PlayerEvent Sender
    event_tx: mpsc::Sender<PlayerOutEvent>,
    mut event_rx: mpsc::Receiver<PlayerInEvent>,
) -> Result<(), ServerError> {
    eprintln!("entered player task");

    let mut local_join_set = JoinSet::<Result<(), ServerError>>::new();

    //(TODO: determine perf implications)
    let (mut conn_read, mut conn_write) = conn.into_split();

    //TODO: determine limit
    let (msg_tx, mut msg_rx) = mpsc::channel::<PlayerEvent>(999);

    //TODO: just require PlayerEvents to be sent?
    // tmp commented out
    /*let msg_recv_task = spawn(cancel_able::<_, Infallible>(cancel_token.child_token(), async move {
        loop {
            msg_tx.send(PlayerEvent::ReceivedEvent(event_rx.recv().await.unwrap()))
                .await
                .unwrap()
        }
    })).await.unwrap();*/

    //TODO: maybe just an atomic?
    let (protocol_stage_tx, mut protocol_stage_rx) = mpsc::channel::<ProtocolStage>(100);

    //same here
    local_join_set.spawn(cancel_able(cancel_token.child_token(), async move {
        eprintln!("prepared reading packets");

        let mut parser = ProtocolParser::new();
        let mut stage = parser.stage().clone();

        loop {
            msg_tx
                .send(
                    PlayerEvent::Packet {
                        packet: parser
                            .parse_packet(&mut DebugReader(&mut conn_read))
                            .await
                            .expect("TODO: proper io error (especially unexpected eof) handling!"),
                        next_stage: *parser.stage(),
                    }, /*PlayerEvent::ReceivedData({
                           let mut buf=Vec::new();
                           conn_read.read_buf(&mut buf).await.unwrap();
                           buf
                       })*/
                )
                .await
                .unwrap();
        }
    }));
    //.await.unwrap()?;

    let (packet_write_tx, mut packet_write_rx) = mpsc::channel(999);

    //maybe blocking on compression via a rt handle??
    local_join_set.spawn(cancel_able(cancel_token.child_token(), async move {
        loop {
            let packet: AnonymousPacket = packet_write_rx.recv().await.unwrap();
            packet
                .send(&mut conn_write)
                .await
                .context(TcpWriteSnafu {})?;
        }
    }));

    local_join_set.spawn(cancel_able(cancel_token, async move {
        //TODO: make steps before play state more types safe
        eprintln!("started main player loop");

        //TODO: synchronize protocol stages
        let mut data = Data::new();

        async fn encode(packet: impl TransferablePacket + Send + 'static) -> AnonymousPacket {
            spawn_blocking(|| AnonymousPacket::new(packet))
                .await
                .unwrap()
        }

        loop {
            match msg_rx.recv().await.expect("channel closed (todo)") {
                //TODO: proper sync with the parsing task
                PlayerEvent::Packet { packet, next_stage } => match packet {
                    InPacket::Handshake(handshake) => {
                        eprintln!("received handshake ");
                        eprintln!("{:?}", handshake);

                        //data.conn.handshake_complete(&handshake);
                        //protocol_stage_tx.send(data.conn.stage.clone()).unwrap();
                    }
                    InPacket::Status(status) => packet_write_tx
                        .send(match status {
                            in_packet::Status::StatusRequest => encode(StatusResponse::new()).await,

                            in_packet::Status::PingRequest(ping) => {
                                encode(PongResponse::new(ping.timestamp())).await
                            }
                        })
                        .await
                        .unwrap(),
                    InPacket::Login(login) => match login {
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                _ => unimplemented!(),
            }
        }
    }));

    //TODO: better solution
    for result in local_join_set.join_next().await.unwrap() {
        if let error @ Err(_) = result {
            return error;
        }
    }
    Ok(())
}
