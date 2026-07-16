pub mod app;
pub mod config;
pub mod msg;
pub mod panel;

pub use msg::Msg;

use crate::config::Config;

//TODO: icon via .with_icon
//for now it's just blocking a server thread...make it a process later
pub fn run(config: Config) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "tachyoo packet viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(eframe_template::PacketViewerApp::new(cc)))),
    )
}
