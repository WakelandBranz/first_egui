use std::sync::{Arc, RwLock};
use eframe::egui;
use serde::{Deserialize, Serialize};
use crate::config::Config;


// This struct is the main TabVisuals that combines settings and runtime config
#[derive(Serialize, Deserialize)]
pub struct TabVisuals {}

impl Default for TabVisuals {
    fn default() -> Self {
        Self {
        }
    }
}

impl TabVisuals {
    pub fn render(&mut self, ui: &mut egui::Ui, config: Arc<RwLock<Config>>) {
        let mut config = config.write().unwrap();
        let box_esp = &mut config.features.box_esp;

        // Checkbox for enabling/disabling the feature
        ui.checkbox(&mut box_esp.enabled, "Box ESP").clicked();

        // Collapsing header for settings (only shown when enabled)
        if box_esp.enabled {
            egui::CollapsingHeader::new("Box ESP Settings")
                .open(Some(box_esp.enabled))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        ui.color_edit_button_rgba_unmultiplied(&mut box_esp.color);
                    });

                    ui.add(egui::Slider::new(&mut box_esp.thickness, 1.0..=10.0)
                        .text("Thickness"));

                    ui.checkbox(&mut box_esp.show_health, "Show Health");
                });
        }
    }
}