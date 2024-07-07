use egui::{Context, TextEdit, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::TemplateApp;

impl TemplateApp {
    pub fn notes(&mut self, ui: &mut Ui, _ctx: &Context) {
        let mut cache = CommonMarkCache::default();
        egui::ScrollArea::vertical().show(ui, |ui| {
            CommonMarkViewer::new("viewer").show(ui, &mut cache, &self.notes_buffer);
        });
    }

    pub fn edit_notes(&mut self, ui: &mut Ui, _ctx: &Context) {
        let theme = egui_extras::syntax_highlighting::CodeTheme::dark();
        let mut layouter = |ui: &Ui, string: &str, _wrap_width: f32| {
            let layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "md");
            ui.fonts(|f| f.layout_job(layout_job))
        };
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut self.notes_buffer)
                    .code_editor()
                    .layouter(&mut layouter),
            );
        });
    }
}
