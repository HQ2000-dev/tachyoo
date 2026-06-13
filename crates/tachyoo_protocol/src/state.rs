#[derive(Default, Debug)]
pub enum ProtocolState {
    Handshake,
    #[default]
    Status,
    Login,
    Config,
    Play,
}
