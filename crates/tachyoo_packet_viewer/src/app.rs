use tachyoo_protocol::{in_::packet::Packet as InPacket, out::packet::Packet as OutPacket};

use crate::{Msg, panel::packet_panel};

pub struct PacketViewerApp {
    in_packets: Vec<InPacket>,
    msg_rx: flume::Receiver<Msg>,
    out_packets: Vec<OutPacket>,
}

impl eframe::App for PacketViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        /*if let Ok(in_packet) = self.in_recv.try_recv() {
            self.in_packets.push(in_packet);
        }

        if let Ok(out_packet) = self.out_recv.try_recv() {
            self.out_packets.push(out_packet);
        }*/

        ui.vertical(|ui| {
            packet_panel(ui, &mut self.in_packets, 10);
            packet_panel(ui, &mut self.out_packets, 10);
        });
    }
}

impl PacketViewerApp {
    pub(crate) fn new(msg_rx: flume::Receiver<Msg>) -> Self {
        Self {
            msg_rx,
            in_packets: Vec::new(),
            out_packets: Vec::new(),
        }
    }
}
