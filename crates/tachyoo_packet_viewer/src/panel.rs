use std::cmp::min;

use egui::Ui;

pub(crate) fn packet_panel<P: Debug>(ui: &mut Ui, packets: &mut Vec<P>, max_shown_packets: u8) {
    packets.iter().rev().take(max_shown_packets as usize).inspect(
        |packet|  {ui.label(packet.to_owned());}
    );
}