
#[derive(Debug, thiserror::Error)]
#[error("the VarInt wouldn't fit into 32 bits")]
pub struct VarIntError(());
#[derive(Debug, thiserror::Error)]
#[error("the VarLong wouldn't fit into 64 bits")]
pub struct VarLongError(());
