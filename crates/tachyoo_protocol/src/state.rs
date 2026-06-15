#[derive(Default, Debug)]
pub enum ProtocolStage {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play,
}
