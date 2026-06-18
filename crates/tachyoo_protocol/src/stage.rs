#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum ProtocolStage {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play,
}
