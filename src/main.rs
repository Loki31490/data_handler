#![allow(unused_variables)]

mod db_handler;
use db_handler::Database;
use eframe::NativeOptions;

mod gui;
use gui::MyApp;

use std::path::PathBuf;

fn main() {
    env_logger::init();

    let path = PathBuf::from("/home/baptiste/Documents/Rust/data_handler/data/my_data.db3");
    let my_database = Database::new(&path);

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "MTE App",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    );
}
