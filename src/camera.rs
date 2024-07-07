use egui::{pos2, Pos2, Rect};

pub struct Camera {
    pan: Pos2,
    pub zoom: f32,
    zoom_center: Pos2,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32, zoom_center_x: f32, zoom_center_y: f32) -> Self {
        Self {
            pan: pos2(x, y),
            zoom,
            zoom_center: pos2(zoom_center_x, zoom_center_y),
        }
    }

    pub fn pan(&mut self, delta: Pos2) {
        self.pan.x += delta.x;
        self.pan.y += delta.y;
    }

    pub fn zoom(&mut self, zoom_factor: f32, mouse_pos: Pos2) {
        let before_zoom_world = self.screen_to_world(mouse_pos);

        self.zoom *= zoom_factor;

        let after_zoom_world = self.screen_to_world(mouse_pos);

        self.pan.x += (after_zoom_world.x - before_zoom_world.x) * self.zoom;
        self.pan.y += (after_zoom_world.y - before_zoom_world.y) * self.zoom;
    }

    pub fn world_to_screen(&self, world: Pos2) -> Pos2 {
        let screen_x = self.zoom_center.x + (world.x - self.zoom_center.x) * self.zoom + self.pan.x;
        let screen_y = self.zoom_center.y + (world.y - self.zoom_center.y) * self.zoom + self.pan.y;
        pos2(screen_x, screen_y)
    }

    pub fn screen_to_world(&self, screen: Pos2) -> Pos2 {
        let world_x = (screen.x - self.zoom_center.x - self.pan.x) / self.zoom + self.zoom_center.x;
        let world_y = (screen.y - self.zoom_center.y - self.pan.y) / self.zoom + self.zoom_center.y;
        pos2(world_x, world_y)
    }

    pub fn rect_with_size(&self, pos: Pos2, width: f32, height: f32) -> Rect {
        Rect::from_min_max(
            self.world_to_screen(pos),
            self.world_to_screen(pos2(pos.x + width, pos.y + height)),
        )
    }
}
