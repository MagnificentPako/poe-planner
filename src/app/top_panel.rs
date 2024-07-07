use egui::Ui;

use crate::TemplateApp;

use super::model::CharacterClass;

impl TemplateApp {
    pub fn top_panel(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Class")
            .selected_text(format!("{:?}", self.selected_class))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_class, CharacterClass::Scion, "Scion");
                ui.selectable_value(
                    &mut self.selected_class,
                    CharacterClass::Marauder,
                    "Marauder",
                );
                ui.selectable_value(&mut self.selected_class, CharacterClass::Ranger, "Ranger");
                ui.selectable_value(&mut self.selected_class, CharacterClass::Witch, "Witch");
                ui.selectable_value(&mut self.selected_class, CharacterClass::Duelist, "Duelist");
                ui.selectable_value(&mut self.selected_class, CharacterClass::Templar, "Templar");
                ui.selectable_value(&mut self.selected_class, CharacterClass::Shadow, "Shadow");
            });
    }
}
