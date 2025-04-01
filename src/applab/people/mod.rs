use crate::applab::HasControlPanel;
use eframe::egui;
use std::fmt;

pub mod attributes;
pub mod errors;

pub struct Person {
    first_name: String,
    last_name: String,
    height: attributes::height::Height,
}
impl Person {
    pub fn new(
        first: &str,
        last: &str,
        height: attributes::height::Height,
    ) -> Self {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            height: height,
        }
    }
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let first = &self.first_name;
        let last = &self.last_name;
        write!(f, "{},{}:{}", last, first, self.height)
    }
}
impl HasControlPanel for Person {
    fn update_readonly(&mut self, ui: &mut egui::Ui) {
        ui.label(" world");
    }
    fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading(" world");
    }
}

// ui.horizontal(|ui| {
//let name_label = ui.label("
// ui.horizontal(|ui| {
//     // let name_label = ui.label("Your name: ");
//     // ui.text_edit_singleline(&mut self.data.name)
//     // .labelled_by(name_label.id);
// });
// ui.add(egui::Slider::new(&mut self.data.age, 0..=120).text("age"));
// if ui.button("+").clicked() {
//     self.data.age += 1;
// }
// ui.label(format!("hello {}, {}", self.data.name, self.data.age));
