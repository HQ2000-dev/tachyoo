use std::{convert::Infallible, pin::Pin, task::Poll};

use tokio::{
    io::{AsyncRead, AsyncReadExt},
    pin, select,
    sync::watch,
};
use tokio_util::sync::CancellationToken;

use crate::ShutdownMsg;

/// a "cancel-able future" (_ -> Result<!, E>) that returns Ok(()) on receiving a cancellation message (_ -> Result<(), E>)
pub async fn cancel_able<F, E>(cancel_token: CancellationToken, fut: F) -> Result<(), E>
where
    F: Future<Output = Result<Infallible, E>>,
{
    select! {
        ret = fut => {
            //unwrap_infallible is unstable
            match ret {
                Err(e) => Err(e),
            }
        },
        //TODO: really working??
        _ = cancel_token.cancelled() => {
            return Ok(());
        }
    }
}

pub struct DebugReader<R: AsyncRead + Unpin>(pub R);

impl<R: AsyncRead + Unpin> AsyncRead for DebugReader<R> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let res = AsyncRead::poll_read(Pin::new(&mut self.0), cx, buf);
        if res.is_ready() {
            //eprintln!("data: {:?}", buf)
        }
        res
    }
}
