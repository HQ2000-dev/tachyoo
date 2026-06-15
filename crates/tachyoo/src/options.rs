//low-level options
pub struct StartOptions {
    pub port: u16,
}

impl Default for StartOptions {
    fn default() -> Self {
        StartOptions { port: 25565 }
    }
}