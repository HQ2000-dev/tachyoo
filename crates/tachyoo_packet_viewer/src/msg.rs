use std::time::Duration;

use tachyoo_protocol::{in_::packet::Packet as InPacket, out::packet::Packet as OutPacket};

//cyclic dep problem!
pub enum Msg {
    Received { packet: InPacket, delta: Duration },
    Sent { packet: OutPacket, delta: OutPacket },
}
