pub mod error;
pub mod options;
pub mod player_data;
pub mod player_task;
pub mod util;

use crate::{
    error::{RuntimeCreationSnafu, ServerError, TcpBindSnafu, TcpConnectSnafu},
    player_task::player_task,
    util::{cancel_able, shutdown_able},
};
use snafu::prelude::*;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use std::net::Ipv6Addr;

use tokio::{
    net::TcpListener, runtime::Handle, sync::{broadcast, mpsc, watch}, task::JoinSet
};

use crate::options::StartOptions;

//TODO: determine HashDoS risk/exposure
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

//TODO maybe use the want crate

pub fn run(options: StartOptions) -> Result<(), ServerError> {
    // hopefully sufficient?
    #[cfg(feature = "tokio_console")]
    console_subscriber::Builder::default()
        .with_default_env()
        .init();

    let rt=tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .context(RuntimeCreationSnafu {})?;

    let handle=rt.handle().clone();

    rt.block_on(run_inner(options, handle));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownMsg(pub bool);

async fn run_inner(options: StartOptions, handle: Handle) -> Result<(), ServerError> {
    //TODO: determine buffer size
    let (conn_tx, conn_rx) = mpsc::channel(100);

    let cancel_token = CancellationToken::new();

    let player_tasks = JoinSet::new();

    //TODO: make tcp connection accepting optionally silently fail
    tokio::spawn(cancel_able(cancel_token, async {
        let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, options.port))
            .await
            .context(TcpBindSnafu {})?;

        loop {
            let (conn, socket_addr) = listener.accept().await.context(TcpConnectSnafu {})?;
            debug!("accepted tcp connection at {}", socket_addr.ip());
            //TODO: player id association
            player_tasks.spawn(player_task(handle.clone(), cancel_token, conn));
        }
    }))
    .await
    // JoinError - panic
    .unwrap()?;

    //TODO: better than this?
    for result in player_tasks.join_all().await {
        result?;
    }

    Ok(())
}
