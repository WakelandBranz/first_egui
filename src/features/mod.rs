// Heavily derived from https://github.com/Valthrun/Valthrun/blob/master/controller/src/enhancements/mod.rs
// Thank you Valthrun, huge help for publishing your project.

pub mod visuals;

use eframe::egui;
use eframe::egui::Painter;
use serde::{Deserialize, Serialize};
use crate::features::visuals::Visuals;
use super::view::world::ViewController;

// Example struct. This should be modified in an actual implementation of this UI
/// Shared data structure that features might need
pub struct GameData {
    pub player_base: usize,
    pub local_player: Entity,
    pub view_controller: ViewController,
    pub entities: Vec<Entity>,

}

impl Default for GameData {
    fn default() -> Self {
        Self {
            player_base: 0x0,
            local_player: Entity::default(),
            view_controller: ViewController::default(),
            entities: Vec::new(),
        }
    }
}

// Another example struct. Used in GameData
#[derive(Default)]
pub struct Entity {
    pos: glam::Vec3,
    view_matrix: glam::Mat4,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Features {
    pub visuals: Visuals,
    // Add other features here
}

// Core feature trait - just the basics
/// Implement for each feature!
pub trait Feature {
    // Used for features like aimbot that DON'T need to be rendered.
    fn update(&mut self, game_data: &GameData) -> anyhow::Result<()>;

    // Updates the struct fields
    fn update_settings(&mut self) -> anyhow::Result<bool> {
        Ok(false)
    }

    // Used for features like ESP that DO need to be rendered.
    fn render(&self, game_data: &GameData, painter: &Painter) -> anyhow::Result<()>;

    // Not always necessary, but can be helpful for quick debug stuff
    fn render_debug(&mut self, game_data: &GameData, ui: &mut egui::Ui) -> anyhow::Result<()>{
        Ok(())
    }
}


