use egui_extras::{self, Column, TableBuilder};

pub struct MyApp {}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let table_builder =
                TableBuilder::new(ui)
                    .columns(Column::auto(), 4)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.checkbox(&mut true, "Select all");
                        });
                        header.col(|ui| {
                            ui.strong("id");
                        });
                        header.col(|ui| {
                            ui.strong("Name");
                        });
                        header.col(|ui| {
                            ui.strong("Last Name");
                        });
                    });
            let table = table_builder.body(|mut body| {
                let mut _select_state = true;
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        if ui.checkbox(&mut _select_state, "").clicked() {};
                    });
                    row.col(|ui| {
                        ui.label("01");
                    });
                    row.col(|ui| {
                        ui.label("Jean");
                    });
                    row.col(|ui| {
                        ui.label("Dujardin");
                    });
                });
            });
        });
    }
}
