use egui::Ui;

use crate::TemplateApp;

use super::model::View;

impl TemplateApp {
    pub fn sidebar(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.button("Tree").clicked() {
                self.selected_view = View::PassiveTree
            };
            if ui.button("Notes").clicked() {
                self.selected_view = View::Note
            }
            if ui.button("Edit Notes").clicked() {
                self.selected_view = View::NoteEdit
            }
            if ui.button("Skills").clicked() {};
            if ui.button("Calcs").clicked() {};
            if ui.button("Party").clicked() {};
        });
        ui.label("Main Skill:");
        let mut selected = "";
        egui::ComboBox::from_id_source("main_skill_combobox")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, "First", "First");
                ui.selectable_value(&mut selected, "Second", "Second");
            });
        ui.separator();
        ui.label(format!("{}", self.camera.zoom));
    }
}
