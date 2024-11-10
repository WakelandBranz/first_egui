use eframe::egui::Context;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use crate::config;
use crate::utils;

#[derive(Serialize, Deserialize, Default)]
pub struct Gui {
    config: config,
    test: String,
}

impl Default for Gui {
    fn default() -> Self {
        Gui {
            config: config::Default(),
            test: "default".to_string(),
        }
    }
}

impl Gui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        todo!()
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        todo!()
    }
}