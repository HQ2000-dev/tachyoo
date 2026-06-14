use std::convert::Infallible;

use tokio::{select, sync::watch};
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
