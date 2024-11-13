use eframe::egui::{Painter, Ui};
// First create your box_esp feature
use serde::{Deserialize, Serialize};
use crate::features::*;

// BoxEsp feature with UI integration
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct NameEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
}

impl Feature for NameEsp {
    fn update(&mut self, data: &GameData) -> anyhow::Result<()> {
        Ok(())
    }

    fn update_settings(&mut self) -> anyhow::Result<bool> {
        todo!()
    }

    fn render(&self, game_data: &GameData, painter: &Painter) -> anyhow::Result<()> {
        for entity in &game_data.entities {

            let screen_pos = game_data.view_controller.world_to_screen(&entity.pos, true)
                .unwrap_or_else(|| {
                    log::error!("Failed to unwrap world to screen function in name_esp::render()!");
                    egui::Pos2::new(0.0, 0.0)
                });

            // painter.text(IMPLEMENT ME PLEASE)
        }

        Ok(())
    }
}
