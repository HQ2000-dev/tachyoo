pub mod data;
pub mod error;
pub mod options;
pub mod player_task;
pub mod util;

use crate::{
    error::{RuntimeCreationSnafu, ServerError, TcpBindSnafu, TcpConnectSnafu},
    player_task::player_task,
    util::cancel_able,
};
use console_subscriber::ConsoleLayer;
use snafu::prelude::*;
#[cfg(feature = "packet_viewer")]
use tachyoo_protocol::{in_::packets::Packet as InPacket, out::packets::Packet as OutPacket};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use tracing_subscriber::util::SubscriberInitExt;

use std::{net::Ipv6Addr, time::Duration};

use tokio::{
    net::TcpListener,
    runtime::Handle,
    sync::{broadcast, mpsc, watch},
    task::JoinSet,
};

use crate::options::StartOptions;

//TODO: determine HashDoS risk/exposure
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

//TODO maybe use the want crate

pub fn run(
    options: StartOptions,
    #[cfg(feature = "packet_viewer")] in_tx: flume::Receiver<InPacket>,
    #[cfg(feature = "packet_viewer")] in_tx: flume::Receiver<OutPacket>,
) -> Result<(), ServerError> {
    // hopefully sufficient?
    #[cfg(feature = "tokio_console")]
    {
        //taken from https://stelfox.net/blog/2023/04/chained-tracing-subscribers/
        // apparently I'm not the only one with this problem
        use std::time::Duration;

        use console_subscriber::ConsoleLayer;
        use tracing::Level;
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;
        use tracing_subscriber::{EnvFilter, Layer};

        let console_layer = ConsoleLayer::builder()
            .retention(Duration::from_secs(30))
            .spawn();

        let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stderr());
        let env_filter = EnvFilter::builder()
            .with_default_directive(Level::INFO.into())
            .from_env_lossy();
        let stderr_layer = tracing_subscriber::fmt::layer()
            .compact()
            .with_writer(non_blocking_writer)
            .with_filter(env_filter);

        tracing_subscriber::registry()
            .with(console_layer)
            .with(stderr_layer)
            .init();
    }

    #[cfg(not(feature = "tokio_console"))]
    tracing_subscriber::FmtSubscriber::new().init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .context(RuntimeCreationSnafu {})?;

    let handle = rt.handle().clone();

    rt.block_on(run_inner(options, handle))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownMsg(pub bool);

async fn run_inner(options: StartOptions, handle: Handle) -> Result<(), ServerError> {
    //TODO: determine buffer size
    //let (conn_tx, conn_rx) = mpsc::channel(100);

    let cancel_token = CancellationToken::new();

    let cloned_cancel_token = cancel_token.clone();

    let mut player_tasks = JoinSet::new();

    let cloned_handle = handle.clone();

    //TODO: make tcp connection accepting optionally silently fail
    tokio::spawn(cancel_able(cancel_token.clone(), async move {
        let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, options.port))
            .await
            .context(TcpBindSnafu {})?;
        //enforce only one status/status-play each at a time! (TODO)
        eprintln!("listening at {}", listener.local_addr().unwrap());

        loop {
            /////
            // tmp, just to make it work
            let (out_event_tx, out_event_rx) = mpsc::channel(100);

            let (in_event_tx, in_event_rx) = mpsc::channel(100);
            ////

            let (conn, peer_addr) = listener.accept().await.context(TcpConnectSnafu {})?;
            eprintln!("accepted tcp connection - peer: {}", peer_addr.ip());
            //TODO: player id association
            player_tasks.spawn(player_task(
                handle.clone(),
                cloned_cancel_token.clone(),
                conn,
                out_event_tx,
                in_event_rx,
            ));
        }
    }))
    .await
    // JoinError - panic
    .unwrap()?;

    //separate task for spawn tasks??
    //TODO: better than this?
    /*for result in player_tasks.join_all().await {
        result?;
    }*/

    Ok(())
}
