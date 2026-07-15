use tachyoo_protocol::{in_::packets::Packet as InPacket, out::packets::Packet as OutPacket};

use crate::panel::packet_panel;

pub struct PacketViewerApp {
    in_packets: Vec<InPacket>,
    in_recv: flume::Receiver<InPacket>,
    out_packets: Vec<OutPacket>,
    out_recv: flume::Receiver<OutPacket>,
}

impl eframe::App for PacketViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if let Ok(in_packet) = self.in_recv.try_recv() {
            self.in_packets.push(in_packet);
        }

        if let Ok(out_packet) = self.out_recv.try_recv() {
            self.out_packets.push(out_packet);
        }
        
        ui.vertical(
            |ui| {
                packet_panel(ui, &mut self.in_packets, 10);
                packet_panel(ui, &mut self.out_packets, 10);
            }
        );
    }
}

impl PacketViewerApp {
    pub(crate) fn new(in_recv: flume::Receiver<InPacket>, in_recv: flume::Receiver<OutPacket>) -> Self {
        Self {
            in_recv,
            out_recv,
            in_packets: Vec::new(),
            out_packets: Vec::new(),
        }
    }
}