use tachyoo_protocol::{in_::packets::Packet as InPacket, out::packets::Packet as OutPacket};

pub enum Msg {
    Received { packet: InPacket, delta: Duration },
    Sent { packet: Packet, delta: OutPacket },
}
