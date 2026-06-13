#[derive(Default, Debug)]
pub enum ProtocolState {
    #[default]
    Initial,
    StatReqRecv,
    StatRespSent,
    PingRecv,
    PongSent,
}
