use std::{process, thread};

//TODO: maybe independent?

#[derive(clap::Parser)]
pub struct CliOptions {
    #[cfg(feature = "packet_viewer")]
    packet_inspector: bool,
}

use clap::Parser;
use tachyoo::{error::ServerError, options::StartOptions};
//#[cfg(feature = "packet_viewer")]
//use tachyoo_protocol::{in_::packets::Packet as InPacket, out::packets::Packet as OutPacket};
#[snafu::report]
fn main() -> Result<(), ServerError> {
    let cli_options = CliOptions::parse();

    //unbounded because we're only using it for development
    #[cfg(feature = "packet_viewer")]
    let (msg_tx, msg_rx) = flume::unbounded();

    #[cfg(feature = "packet_viewer")]
    if cli_options.packet_inspector {
        thread::spawn(|| tachyoo_packet_viewer::run(config, msg_rx));
    }

    let options = StartOptions::default();
    tachyoo::run(
        options,
        #[cfg(feature = "packet_viewer")]
        msg_tx,
    )
}
