use std::io;

use snafu::prelude::*;

//fatal server error
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ServerError {
    #[snafu(display("Failed to create a tokio runtime"))]
    RuntimeCreation {
        source: io::Error,
    },
    #[snafu(display("Failed to bind to a tcp socket"))]
    TcpBind {
        source: io::Error,
        //socket_addr: SocketAddr,
    },
    #[snafu(display("Failed to establish a new tcp connection"))]
    TcpConnect {
        source: io::Error,
    },
    #[snafu(display("Failed to write to a tcp stream"))]
    TcpWrite {
        source: io::Error,
    },
    /* #[snafu(display("Failed to read from a tcp stream"))]
    TcpRead {
        source: io::Error,
    },
    #[snafu(display("Failed to read from a tcp stream"))]
    TcpWrite {
        source: io::Error,
    },*/
    InvalidSocketAddress {
        source: io::Error,
        port: u16,
    },
}
