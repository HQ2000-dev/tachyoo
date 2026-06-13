pub mod error;
pub mod options;
pub mod player_data;
pub mod util;
pub mod player_task;

use crate::{error::{RuntimeCreationSnafu, ServerError, TcpBindSnafu, TcpConnectSnafu}, player_task::player_task, util::{never, shutdown_able}};
use console_subscriber::Server;
use snafu::prelude::*;
use tracing::debug;

use std::net::Ipv6Addr;

use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc}, task::JoinSet,
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

    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .context(RuntimeCreationSnafu {})?
        .block_on(run_inner(options))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownMsg;

async fn run_inner(options: StartOptions) -> Result<(), ServerError> {
    //TODO: determine buffer size
    let (conn_tx, conn_rx) = mpsc::channel(100);

    let (shutdown_tx, shutdown_rx) = broadcast::channel::<ShutdownMsg>(1);

    

    let player_tasks=JoinSet::new();
    
    //TODO: make tcp connection accepting optionally silently fail
    tokio::spawn(shutdown_able(shutdown_rx, async {
        let listener = TcpListener::bind((Ipv6Addr::LOCALHOST, options.port))
            .await
            .context(TcpBindSnafu {})?;

        loop {
            let (conn, socket_addr) = listener.accept().await.context(TcpConnectSnafu {})?;
            debug!("accepted tcp connection at {}", socket_addr.ip());
            player_tasks.spawn(shutdown_able(shutdown_rx, player_task(conn)));
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
