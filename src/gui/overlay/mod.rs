mod draw;
pub mod thread;

use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use eframe::egui::Context;
use eframe::{egui, Frame, Storage};
use serde::{Deserialize, Serialize};
use crate::config::*;
use crate::features::{Feature, GameData};
use crate::utils;

pub struct Overlay {
    config: Arc<RwLock<Config>>,
    game_data: Arc<RwLock<GameData>>
}

impl Overlay {
    /// Called once before the first frame.
    pub fn new(config: Arc<RwLock<Config>>, game_data: Arc<RwLock<GameData>>, _cc: &eframe::CreationContext) -> Self {
        Self {
            config: config.clone(),
            game_data: game_data.clone(),
        }
    }

    fn render_content(&mut self, ui: &mut egui::Ui) {
        let config = self.config.read().unwrap();
        let game_data = self.game_data.read().unwrap();

        let painter = ui.painter();

        let box_esp = &config.features.visuals.box_esp;

        if box_esp.enabled {
            box_esp.render(&game_data, painter).unwrap()
        }

        let name_esp = &config.features.visuals.name_esp;

        if name_esp.enabled {
            name_esp.render(&game_data, painter).unwrap()
        }
    }
}

// Main rendering loop
impl eframe::App for Overlay {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Main render loop

        // Create a fullscreen area that will contain our overlay
        egui::Area::new(egui::Id::from("overlay_area"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .interactable(false)
            .show(ctx, |ui| {
                self.render_content(ui)
            });

    }

    fn save(&mut self, storage: &mut dyn Storage) {
        // Save the config
        self.config.write().unwrap().save().expect("Failed to save config on exit!");

    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("Application exiting...");

        // Note: eframe calls save on exit
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(120)
    }
}