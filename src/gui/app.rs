use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;

pub struct MyTestForm {
    name: String,
    Last_name: String,
}

#[derive(Default)]
pub struct Appstate(String);

pub struct AppConfig {
    window_size: egui::Vec2,
}

pub struct MyApp {
    state: Appstate,
    database: FileDialog,
    config: AppConfig,
    test_form: MyTestForm,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: Appstate::default(),
            database: FileDialog::new(),
            config: AppConfig {
                window_size: egui::vec2(800.0, 600.0),
            },

            test_form: MyTestForm {
                name: String::default(),
                Last_name: String::default(),
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Data Handler");
                ui.separator();

                ui.label("Name");
                ui.text_edit_singleline(&mut self.test_form.name);

                ui.label("Last Name");
                ui.text_edit_singleline(&mut self.test_form.Last_name);

                ui.separator();
                if ui.button("Add").clicked() {
                    todo!()
                };
            });

            let table_builder =
                TableBuilder::new(ui)
                    .columns(Column::auto(), 4)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.checkbox(&mut true, "");
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
