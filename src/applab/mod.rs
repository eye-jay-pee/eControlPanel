use eframe::*;
use std::any;

pub mod errors;
pub mod people;

pub trait HasControlPanel: std::fmt::Display {
    fn update_readonly(&self, ui: &mut egui::Ui);
    fn update(&mut self, ui: &mut egui::Ui);
}

pub struct TestApp<T: HasControlPanel> {
    data: T,
}

impl<T: HasControlPanel> TestApp<T> {
    pub fn new(data: T) -> Self {
        TestApp { data }
    }
    pub fn launch(self) -> errors::Result {
        let app_title = any::type_name_of_val(&self.data);
        let app_options = NativeOptions::default();
        let app_creator: AppCreator = Box::new(|_cc| Ok(Box::new(self)));

        match run_native(app_title, app_options, app_creator) {
            Ok(()) => Ok(()),
            Err(e) => Err(errors::AppLabError::EFrameError(e)),
        }
    }
}
impl<T: HasControlPanel> eframe::App for TestApp<T> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Editable:");
            self.data.update(ui);
            ui.separator();
            ui.heading("Readonly:");
            self.data.update_readonly(ui);
        });
    }
}
