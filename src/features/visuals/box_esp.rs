use eframe::egui::Ui;
// First create your box_esp feature
use serde::{Deserialize, Serialize};
use crate::features::*;

// BoxEsp feature with UI integration
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct BoxEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub thickness: f32,
    pub show_health: bool,
}

impl Feature for BoxEsp {
    // Not needed for BoxEsp (this requires rendering!)
    fn update(&mut self, data: &GameData) -> anyhow::Result<()> {
        Ok(())
    }

    fn update_settings(&mut self) -> anyhow::Result<bool> {
        todo!()
    }

    fn render(&mut self, game_data: &GameData, ui: &mut Ui) -> anyhow::Result<()> {
        for entity in &game_data.entities {
            
            let screen_pos = &game_data.view_controller.world_to_screen(
                &entity.pos,
                true,
            );
            
            // Implement box rendering here!
        }

        Ok(())
    }
}
