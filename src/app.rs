use std::f32::consts::PI;

use crate::tree::{self, Group, Node, Sprite, SpriteCoords, Spritesheet};
use egui::{pos2, util::History, Color32, Pos2, Rect, TextureId};

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
    skills_tex: TextureId,
    frame_tex: TextureId,
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
            skills_tex: TextureId::User(999),
            frame_tex: TextureId::User(999),
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

        let skills_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://skills-3.png".into(),
            include_bytes!("../resources/ggg_assets/skills-3.png"),
        );

        let frame_tex = image_as_texture(
            &cc.egui_ctx,
            "bytes://frame-3.png".into(),
            include_bytes!("../resources/ggg_assets/frame-3.png"),
        );

        TemplateApp {
            group_bg_tex,
            bg_tex,
            skills_tex,
            frame_tex,
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

fn orbit_position(node: &Node, group: &Group) -> [f32; 2] {
    let radius = tree::ORBIT_RADII[node.orbit.unwrap_or(0)] as f32;
    let skills_on_orbit = tree::ORBIT_NODES[node.orbit.unwrap_or(0)];
    let orbit_index = node.orbit_index.unwrap_or(0);
    let two_pi = PI * 2.0;

    let angle = match skills_on_orbit {
        16 => (tree::ORBIT_ANGLES_16[orbit_index] as f32).to_radians(),
        40 => (tree::ORBIT_ANGLES_40[orbit_index] as f32).to_radians(),
        an => two_pi / an as f32 * orbit_index as f32,
    };

    let x = group.x + radius * angle.sin();
    let y = group.y - radius * angle.cos();

    [x, y]
}

fn sprite_uv(sheet: &Sprite, coord: &SpriteCoords) -> Rect {
    Rect::from_min_max(
        pos2((coord.x / sheet.w) as f32, (coord.y / sheet.h) as f32),
        pos2(
            (coord.x + coord.w) as f32 / sheet.w as f32,
            (coord.y + coord.h) as f32 / sheet.h as f32,
        ),
    )
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
                    if ui.button("Tree").clicked() {};
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

            for group in self.tree.groups.values() {
                if let Some(bg) = &group.background {
                    let half_image = bg.is_half_image.unwrap_or(false);
                    let spritesheet = &self.tree.sprites.group_background.sprites;
                    if let Some(sprite) = spritesheet.coords.get(&bg.image) {
                        let offset = match half_image {
                            true => ((sprite.w / 2) as f32 * -1.0, sprite.h as f32 * -1.0),
                            false => ((sprite.w / 2) as f32 * -1.0, (sprite.h / 2) as f32 * -1.0),
                        };
                        painter.image(
                            self.group_bg_tex,
                            Rect::from_min_max(
                                to_screen.transform_pos(Pos2::new(
                                    group.x + offset.0,
                                    group.y + offset.1,
                                )),
                                to_screen.transform_pos(Pos2::new(
                                    group.x + sprite.w as f32 + offset.0,
                                    group.y + sprite.h as f32 + offset.1,
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
                                        group.x + offset.0,
                                        group.y + sprite.h as f32 + offset.1,
                                    )),
                                    to_screen.transform_pos(Pos2::new(
                                        group.x + sprite.w as f32 + offset.0,
                                        group.y + sprite.h as f32 + sprite.h as f32 + offset.1,
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

            let normal_active = &self.tree.sprites.normal_active.sprites;

            for node in self.tree.nodes.values() {
                if node.is_proxy.unwrap_or(false) {
                    continue;
                }
                if let Some(icon) = &node.icon {
                    if let Some(sprite_info) = normal_active.coords.get(icon) {
                        if let Some(group_num) = node.group {
                            let group_str = format!("{group_num}");
                            let group = self.tree.groups.get(&group_str).unwrap();
                            let orbitpos = orbit_position(node, group);
                            let opos = [
                                orbitpos[0] - sprite_info.w as f32 / 2.0,
                                orbitpos[1] - sprite_info.h as f32 / 2.0,
                            ];
                            let scalar = match node.is_notable.unwrap_or(false) {
                                true => 2.0,
                                false => 1.0,
                            };
                            painter.image(
                                self.skills_tex,
                                Rect::from_min_max(
                                    to_screen.transform_pos(pos2(opos[0], opos[1])),
                                    to_screen.transform_pos(pos2(
                                        opos[0] + (sprite_info.w as f32 * scalar),
                                        opos[1] + (sprite_info.h as f32 * scalar),
                                    )),
                                ),
                                Rect::from_min_max(
                                    pos2(
                                        sprite_info.x as f32 / normal_active.w as f32,
                                        sprite_info.y as f32 / normal_active.h as f32,
                                    ),
                                    pos2(
                                        (sprite_info.x as f32 + sprite_info.w as f32)
                                            / normal_active.w as f32,
                                        (sprite_info.y as f32 + sprite_info.h as f32)
                                            / normal_active.h as f32,
                                    ),
                                ),
                                Color32::WHITE,
                            );
                            if node.is_notable.unwrap_or(false) {
                                let frames = &self.tree.sprites.frame.sprites.coords;
                                let notable_frame = frames.get("NotableFrameUnallocated").unwrap();
                                painter.image(
                                    self.frame_tex,
                                    Rect::from_min_max(
                                        to_screen.transform_pos(pos2(opos[0], opos[1])),
                                        to_screen.transform_pos(pos2(
                                            opos[0] + notable_frame.w as f32,
                                            opos[1] + notable_frame.h as f32,
                                        )),
                                    ),
                                    Rect::from_min_max(
                                        pos2(
                                            notable_frame.x as f32
                                                / self.tree.sprites.frame.sprites.w as f32,
                                            notable_frame.y as f32
                                                / self.tree.sprites.frame.sprites.h as f32,
                                        ),
                                        pos2(
                                            (notable_frame.x + notable_frame.w) as f32
                                                / self.tree.sprites.frame.sprites.w as f32,
                                            (notable_frame.y + notable_frame.h) as f32
                                                / self.tree.sprites.frame.sprites.h as f32,
                                        ),
                                    ),
                                    Color32::WHITE,
                                );
                            }
                        }
                    }
                }
            }
        });
    }
}
