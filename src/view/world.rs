// Thank you Valthrun https://github.com/Valthrun/Valthrun/blob/aa316373a7769cb4fe6686f99f50e1596cf15128/controller/src/view/world.rs

use eframe::egui;

#[derive(Default)]
/// View controller which helps resolve in game
/// coordinates into 2d screen coordinates.
pub struct ViewController {
    view_matrix: glam::Mat4,
    pub screen_bounds: glam::Vec2,
}

impl ViewController {
    pub fn update_screen_bounds(&mut self, bounds: glam::Vec2) {
        self.screen_bounds = bounds;
    }

    pub fn world_to_screen(
        &self,
        pos: &glam::Vec3,
        allow_off_screen: bool,
    ) -> Option<egui::Pos2> {
        let screen_coords = self.view_matrix * glam::vec4(pos.x, pos.y, pos.z, 1.0);

        if screen_coords.w < 0.1 {
            return None;
        }

        if !allow_off_screen
            && (screen_coords.x < -screen_coords.w
            || screen_coords.x > screen_coords.w
            || screen_coords.y < -screen_coords.w
            || screen_coords.y > screen_coords.w)
        {
            return None;
        }

        let mut screen_pos = glam::vec2(
            screen_coords.x / screen_coords.w,
            screen_coords.y / screen_coords.w,
        );

        screen_pos.x = (screen_pos.x + 1.0) * self.screen_bounds.x / 2.0;
        screen_pos.y = (-screen_pos.y + 1.0) * self.screen_bounds.y / 2.0;

        // The following is for egui overlays only. Return Some(screen_pos) for Vec2.
        Some(egui::Pos2::new(screen_pos.x, screen_pos.y))
    }
}