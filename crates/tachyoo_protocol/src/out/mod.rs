pub(super) mod raw_packet;
pub mod types;

//types that are reprs for transmission
pub trait Transfer: AsRef<[u8]> {}

impl<T> Transfer for T where T: AsRef<[u8]> {}
