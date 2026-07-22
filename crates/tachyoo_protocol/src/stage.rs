#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolStage {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play,
}
