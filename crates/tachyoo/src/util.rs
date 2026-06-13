use std::convert::Infallible;

use tokio::{select, sync::broadcast};

use crate::ShutdownMsg;

/// a "shutdownable future" (_ -> Result<!, E>) that returns Ok(()) on receiving a shutdown message (_ -> Result<(), E>)
pub async fn shutdown_able<F, E>(
    mut shutdown_rx: broadcast::Receiver<ShutdownMsg>,
    fut: F,
) -> Result<(), E>
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
        Ok(_) = shutdown_rx.recv() => {
            return Ok(());
        }
    }
}