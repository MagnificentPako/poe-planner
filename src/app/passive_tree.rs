use egui::{pos2, Color32, Context, Rect, Stroke, Ui};

use crate::{
    tree::{FrameType, CLASS_ART},
    TemplateApp,
};

use super::utility::draw_asset;

impl TemplateApp {
    pub fn passive_tree(&mut self, ui: &mut Ui, ctx: &Context) {
        let available_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(available_size, egui::Sense::drag());
        let mut hovered_node = "";

        if response.dragged() {
            self.camera.pan(response.drag_delta().to_pos2());
        }

        let zoom_factor = 1.1;
        if response.hovered() {
            if ctx.input(|i| i.raw_scroll_delta.y != 0.0) {
                if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    if ctx.input(|i| i.raw_scroll_delta.y > 0.0) {
                        self.camera.zoom(zoom_factor, mouse_pos);
                    } else if ctx.input(|i| i.raw_scroll_delta.y < 0.0) {
                        self.camera.zoom(zoom_factor.recip(), mouse_pos);
                    }
                }
            }

            if let Some(hover_pos) = response.hover_pos() {
                for (id, node) in &self.tree.nodes {
                    let node_pos = self.tree.node_position(node);
                    let screen_pos = self.camera.world_to_screen(pos2(node_pos.0, node_pos.1));
                    let distance = screen_pos.distance(hover_pos);
                    if distance < 50.0 * self.camera.zoom {
                        hovered_node = id;
                    }
                }
            }
        }

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
                let spritesheet = &self.tree.sprites.group_background.sprites;
                if let Some(sprite) = spritesheet.coords.get(&bg.image) {
                    draw_asset(
                        sprite,
                        spritesheet,
                        self.group_bg_tex,
                        pos2(group.x, group.y),
                        bg.is_half_image.unwrap_or(false),
                        &painter,
                        &self.camera,
                    );
                }
            }
        }

        for (start_point, tex_name) in self.tree.get_ascendancy_starts() {
            let spritesheet = &self.tree.sprites.ascendancy_background.worse_sprites;
            if let Some(sprite) = spritesheet.coords.get(&tex_name) {
                draw_asset(
                    sprite,
                    spritesheet,
                    self.ascendancy_tex,
                    start_point,
                    false,
                    &painter,
                    &self.camera,
                );
            }
        }

        for line in &self.lines {
            let from = self.camera.world_to_screen(line.0);
            let to = self.camera.world_to_screen(line.1);

            painter.line_segment(
                [from, to],
                Stroke::new(5.0 * self.camera.zoom, Color32::WHITE),
            );
        }

        let normal_active = &self.tree.sprites.normal_active.sprites;
        let mastery_inactive = &self.tree.sprites.mastery_inactive.sprites;

        for (node_id, node) in &self.tree.nodes {
            if let Some(class_start_index) = node.class_start_index {
                let tex_name = if class_start_index == self.selected_class.id() as usize {
                    CLASS_ART[class_start_index]
                } else {
                    "PSStartNodeBackgroundInactive"
                };
                let node_pos = self.tree.node_position(node);
                let sprite_info = self
                    .tree
                    .sprites
                    .start_node
                    .sprites
                    .coords
                    .get(tex_name)
                    .unwrap();
                draw_asset(
                    sprite_info,
                    &self.tree.sprites.start_node.sprites,
                    self.group_bg_tex,
                    pos2(node_pos.0, node_pos.1),
                    false,
                    &painter,
                    &self.camera,
                );
                continue;
            }
            if node.is_proxy.unwrap_or(false) {
                continue;
            }
            if node.is_ascendancy_start {
                continue;
            }
            let node_is_hovered = node_id == hovered_node;
            if node.is_mastery {
                if let Some(sprite_info) = mastery_inactive.coords.get(&node.inactive_icon) {
                    let nodepos = self.tree.node_position(node);
                    draw_asset(
                        sprite_info,
                        mastery_inactive,
                        self.mastery_tex,
                        pos2(nodepos.0, nodepos.1),
                        false,
                        &painter,
                        &self.camera,
                    );
                }
                continue;
            }
            if let Some(icon) = &node.icon {
                let appropriate_sheet = match node.frame_type() {
                    FrameType::Keystone => &self.tree.sprites.keystone_active.sprites,
                    FrameType::Notable => &self.tree.sprites.notable_active.sprites,
                    FrameType::Normal => normal_active,
                    _ => normal_active,
                };
                if let Some(sprite_info) = appropriate_sheet.coords.get(icon) {
                    let nodepos = self.tree.node_position(node);
                    let tex = match node_id == hovered_node {
                        true => self.skills_tex,
                        false => self.inactive_skills_tex,
                    };

                    draw_asset(
                        sprite_info,
                        appropriate_sheet,
                        tex,
                        pos2(nodepos.0, nodepos.1),
                        false,
                        &painter,
                        &self.camera,
                    );
                    let frame_name_opt = match node.frame_type() {
                        FrameType::Keystone => {
                            if node_is_hovered {
                                Some("KeystoneFrameCanAllocate")
                            } else {
                                Some("KeystoneFrameUnallocated")
                            }
                        }
                        FrameType::Notable => {
                            if node_is_hovered {
                                Some("NotableFrameAllocated")
                            } else {
                                Some("NotableFrameCanAllocate")
                            }
                        }
                        FrameType::Normal => {
                            if node_is_hovered {
                                Some("PSSkillFrameHighlighted")
                            } else {
                                Some("PSSkillFrame")
                            }
                        }
                        _ => None,
                    };
                    if let Some(frame_name) = frame_name_opt {
                        let frame = self
                            .tree
                            .sprites
                            .frame
                            .sprites
                            .coords
                            .get(frame_name)
                            .unwrap();
                        draw_asset(
                            frame,
                            &self.tree.sprites.frame.sprites,
                            self.frame_tex,
                            pos2(nodepos.0, nodepos.1),
                            false,
                            &painter,
                            &self.camera,
                        );
                    }
                }
            }
        }
    }
}
