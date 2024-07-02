use crate::tree::{self, ORBIT_NODES};
use eframe::glow::Texture;
use egui::{
    epaint::util::OrderedFloat, pos2, util::History, Align2, Color32, Context, Pos2, Rect,
    TextureId,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip_serializing)]
    zoom: f32,
    #[serde(skip_serializing)]
    pan: egui::Vec2,
    #[serde(skip_serializing)]
    tree: tree::TreeExport,
    #[serde(skip_serializing)]
    group_bg_tex: TextureId,
    #[serde(skip_serializing)]
    bg_tex: TextureId,
    frame_times: History<f32>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;
        Self {
            zoom: 1.0,
            pan: egui::Vec2::new(0.0, 0.0),
            tree: tree::TreeExport::new().unwrap(),
            group_bg_tex: TextureId::User(999),
            bg_tex: TextureId::User(999),
            frame_times: History::new(0..max_len, max_age),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        egui_extras::install_image_loaders(&cc.egui_ctx);

        let group_bg_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://group-background-3.png".into(),
            include_bytes!("../resources/ggg_assets/group-background-3.png"),
        );

        let bg_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://background-3.png".into(),
            include_bytes!("../resources/ggg_assets/background-3.png"),
        );

        TemplateApp {
            group_bg_tex: group_bg_tex,
            bg_tex: bg_tex,
            ..Default::default()
        }
    }
}

fn image_as_texture(ctx: &egui::Context, uri: String, bytes: &'static [u8]) -> TextureId {
    ctx.include_bytes(uri.clone(), bytes);
    ctx.try_load_texture(&uri, Default::default(), egui::SizeHint::Scale(1.0.into()))
        .unwrap()
        .texture_id()
        .unwrap()
}

fn orbit_position(x: f32, y: f32, radius: f32, offset: usize, orbit: usize) -> [f32; 2] {
    if orbit == 0 {
        [x, y]
    } else if orbit == 2 || orbit == 3 {
        [
            radius * (tree::ORBIT_ANGLES_16[offset] as f32).to_radians().sin() + x,
            radius * (tree::ORBIT_ANGLES_16[offset] as f32).to_radians().cos() + y,
        ]
    } else {
        [
            radius
                * ((360 / tree::ORBIT_NODES[orbit]) as f32)
                * (offset as f32 + tree::ORBIT_NODES[orbit] as f32 / 2.0) as f32
                + x,
            radius
                * ((360 / tree::ORBIT_NODES[orbit]) as f32)
                * (offset as f32 + tree::ORBIT_NODES[orbit] as f32 / 2.0) as f32
                + y,
        ]
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        let frame_time = frame.info().cpu_usage.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = frame_time;
        }
        self.frame_times.add(now, frame_time);
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            let _ = ui.button(":)");
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(250.0)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    ui.button("Tree");
                    ui.button("Skills");
                    ui.button("Calcs");
                    ui.button("Party");
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
                ui.label(format!("{}", self.zoom));
                ui.label(format!("{:?}", self.pan));
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_height(10.0);
                ui.label("Bottom Panel");
                let _ = ui.button("Buttom Button");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::drag());

            if response.dragged() {
                self.pan -= response.drag_delta() * (self.zoom);
            }

            let zoom_factor = 1.1;
            if response.hovered() {
                if ctx.input(|i| i.raw_scroll_delta.y > 0.0) {
                    self.zoom /= zoom_factor;
                } else if ctx.input(|i| i.raw_scroll_delta.y < 0.0) {
                    self.zoom *= zoom_factor;
                }
            }

            let to_screen = egui::emath::RectTransform::from_to(
                egui::Rect::from_min_max(
                    Pos2::default() + self.pan,
                    Pos2::default() + self.pan + response.rect.size() * self.zoom,
                ),
                response.rect,
            );

            painter.image(
                self.bg_tex,
                Rect::from_min_max(
                    pos2(0.0, 0.0),
                    pos2(available_size.x + 300.0, available_size.y + 100.0),
                ),
                Rect::from_min_max(
                    pos2(0.0, 0.0),
                    pos2(available_size.x / 89.0, available_size.y / 89.0),
                ),
                Color32::from_rgb(255, 255, 255),
            );

            for group in self.tree.groups.values().into_iter() {
                if let Some(bg) = &group.background {
                    let half_image = bg.is_half_image.or(Some(false)).unwrap();
                    let spritesheet = self
                        .tree
                        .sprites
                        .get("groupBackground")
                        .unwrap()
                        .get("0.3835")
                        .unwrap();
                    if let Some(sprite) = spritesheet.coords.get(&bg.image) {
                        painter.image(
                            self.group_bg_tex,
                            Rect::from_min_max(
                                to_screen.transform_pos(Pos2::new(group.x, group.y)),
                                to_screen.transform_pos(Pos2::new(
                                    group.x + sprite.w as f32,
                                    group.y + sprite.h as f32,
                                )),
                            ),
                            Rect::from_min_max(
                                pos2(
                                    sprite.x as f32 / spritesheet.w as f32,
                                    sprite.y as f32 / spritesheet.h as f32,
                                ),
                                pos2(
                                    (sprite.x as f32 + sprite.w as f32) / spritesheet.w as f32,
                                    (sprite.y as f32 + sprite.h as f32) / spritesheet.h as f32,
                                ),
                            ),
                            Color32::from_rgb(255, 255, 255),
                        );
                        if half_image {
                            painter.image(
                                self.group_bg_tex,
                                Rect::from_min_max(
                                    to_screen.transform_pos(Pos2::new(
                                        group.x,
                                        group.y + sprite.h as f32,
                                    )),
                                    to_screen.transform_pos(Pos2::new(
                                        group.x + sprite.w as f32,
                                        group.y + sprite.h as f32 + sprite.h as f32,
                                    )),
                                ),
                                Rect::from_min_max(
                                    pos2(
                                        sprite.x as f32 / spritesheet.w as f32,
                                        (sprite.y as f32 + sprite.h as f32) / spritesheet.h as f32,
                                    ),
                                    pos2(
                                        (sprite.x as f32 + sprite.w as f32) / spritesheet.w as f32,
                                        sprite.y as f32 / spritesheet.h as f32,
                                    ),
                                ),
                                Color32::from_rgb(255, 255, 255),
                            );
                        }
                    }
                }
            }

            for node in self.tree.nodes.values().into_iter() {
                if let Some(_) = &node.icon {
                    if let Some(orbit) = node.orbit {
                        let group_num = node.group.unwrap();
                        let group_str = format!("{group_num}");
                        let group = self.tree.groups.get(&group_str).unwrap();
                        let spritesheet = self
                            .tree
                            .sprites
                            .get("groupBackground")
                            .unwrap()
                            .get("0.3835")
                            .unwrap();
                        if let Some(bg) = &group.background {
                            if let Some(sprite) = spritesheet.coords.get(&bg.image) {
                                let is_half_image = bg.is_half_image.or(Some(false)).unwrap();
                                let opos = orbit_position(
                                    group.x + (sprite.w as f32 / 2.0),
                                    group.y
                                        + (sprite.h as f32 / 2.0)
                                        + if is_half_image {
                                            sprite.h as f32 / 2.0
                                        } else {
                                            0.0
                                        },
                                    self.tree.constants.orbit_radii[orbit] as f32,
                                    node.orbit_index.unwrap_or(0),
                                    orbit,
                                );
                                painter.circle_filled(
                                    to_screen.transform_pos(Pos2::new(opos[0], opos[1])),
                                    25.0 * (1.0 / self.zoom),
                                    Color32::from_rgb(0, 255, 0),
                                );
                            }
                        }
                    }
                }
            }
        });

        let dbgp = ctx.debug_painter();
        dbgp.debug_text(
            pos2(20.0, 20.0),
            Align2::LEFT_TOP,
            Color32::from_rgb(255, 255, 255),
            format!(
                "FPS: {}",
                1.0 / self.frame_times.mean_time_interval().unwrap_or_default()
            ),
        );
    }
}
