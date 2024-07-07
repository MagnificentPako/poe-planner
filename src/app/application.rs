use crate::{tree, TemplateApp};

use super::{model::View, utility::image_as_texture};

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let group_bg_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://group-background-3.png".into(),
            include_bytes!("../../resources/ggg_assets/group-background-3.png"),
        );

        let bg_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://background-3.png".into(),
            include_bytes!("../../resources/ggg_assets/background-3.png"),
        );

        let skills_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://skills-3.png".into(),
            include_bytes!("../../resources/ggg_assets/skills-3.png"),
        );

        let frame_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://frame-3.png".into(),
            include_bytes!("../../resources/ggg_assets/frame-3.png"),
        );

        let inactive_skills_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://skills-disabled-3.jpg".into(),
            include_bytes!("../../resources/ggg_assets/skills-disabled-3.jpg"),
        );

        let mastery_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://mastery-disabled-3.png".into(),
            include_bytes!("../../resources/ggg_assets/mastery-disabled-3.png"),
        );

        let ascendancy_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://ascendancy-background-2.jpg".into(),
            include_bytes!("../../resources/ggg_assets/ascendancy-background-2.jpg"),
        );

        let tree = tree::TreeExport::new().unwrap();
        let lines = tree.generate_lines();

        TemplateApp {
            group_bg_tex,
            bg_tex,
            skills_tex,
            frame_tex,
            inactive_skills_tex,
            mastery_tex,
            ascendancy_tex,
            lines,
            ..Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        let frame_time = frame.info().cpu_usage.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = frame_time;
        }
        self.frame_times.add(now, frame_time);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.top_panel(ui);
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(250.0)
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_height(10.0);
                ui.label("Bottom Panel");
                let _ = ui.button("Buttom Button");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_view {
                View::PassiveTree => self.passive_tree(ui, ctx),
                View::Note => self.notes(ui, ctx),
                View::NoteEdit => self.edit_notes(ui, ctx),
            };
        });
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        raw_input.max_texture_side = Some(4096);
    }
}
