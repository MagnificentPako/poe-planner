use egui::{pos2, Color32, Painter, Pos2, Rect, TextureId, TextureOptions};

use crate::{
    camera::Camera,
    tree::{Sprite, SpriteCoords},
};

pub fn draw_asset(
    sprite: &SpriteCoords,
    sheet: &Sprite,
    texture: TextureId,
    pos: Pos2,
    is_half: bool,
    painter: &Painter,
    camera: &Camera,
) {
    let width = sprite.w as f32 * 1.33;
    let height = sprite.h as f32 * 1.33;
    if is_half {
        let top_rect = camera.rect_with_size(
            pos2(pos.x - width, pos.y - height * 2.0),
            width * 2.0,
            height * 2.0,
        );
        let bottom_rect =
            camera.rect_with_size(pos2(pos.x - width, pos.y), width * 2.0, height * 2.0);
        let top_uv = Rect::from_min_max(
            pos2(
                sprite.x as f32 / sheet.w as f32,
                sprite.y as f32 / sheet.h as f32,
            ),
            pos2(
                (sprite.x + sprite.w) as f32 / sheet.w as f32,
                (sprite.y + sprite.h) as f32 / sheet.h as f32,
            ),
        );
        let bottom_uv = Rect::from_min_max(
            pos2(
                sprite.x as f32 / sheet.w as f32,
                (sprite.y + sprite.h) as f32 / sheet.h as f32,
            ),
            pos2(
                (sprite.x + sprite.w) as f32 / sheet.w as f32,
                sprite.y as f32 / sheet.h as f32,
            ),
        );
        painter.image(texture, top_rect, top_uv, Color32::WHITE);
        painter.image(texture, bottom_rect, bottom_uv, Color32::WHITE);
    } else {
        painter.image(
            texture,
            Rect::from_min_max(
                camera.world_to_screen(pos2(pos.x - width, pos.y - height)),
                camera.world_to_screen(pos2(pos.x + width, pos.y + height)),
            ),
            Rect::from_min_max(
                pos2(
                    sprite.x as f32 / sheet.w as f32,
                    sprite.y as f32 / sheet.h as f32,
                ),
                pos2(
                    (sprite.x + sprite.w) as f32 / sheet.w as f32,
                    (sprite.y + sprite.h) as f32 / sheet.h as f32,
                ),
            ),
            Color32::WHITE,
        );
    }
}

pub fn image_as_texture(ctx: &egui::Context, uri: String, bytes: &'static [u8]) -> TextureId {
    let texture_options = TextureOptions {
        magnification: egui::TextureFilter::Nearest,
        minification: egui::TextureFilter::Nearest,
        wrap_mode: egui::TextureWrapMode::Repeat,
    };
    ctx.include_bytes(uri.clone(), bytes);
    ctx.try_load_texture(&uri, texture_options, egui::SizeHint::Scale(1.0.into()))
        .unwrap()
        .texture_id()
        .unwrap()
}
