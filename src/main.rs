#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod utils;
mod gui;
mod config;
mod features;
mod view;

use std::sync::{Arc, RwLock};
use eframe::egui;
use crate::config::Config;
use crate::features::GameData;
use crate::gui::menu::Menu;
use crate::gui::{menu, overlay};

fn main() -> eframe::Result {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp_secs()
        .init();

    // Load config from file or create default
    let mut config = Arc::new(RwLock::new(
        Config::load().unwrap_or_else(|e| {
            log::error!("Failed to load config: {}, using default", e);
            Config::default()
        })));

    // TODO!
    // Implement a check here to see if the game is open!!!

    let mut game_data = Arc::new(RwLock::new(GameData::default()));

    // Clone the Arcs for the overlay thread
    let mut overlay_config = config.clone();
    let mut overlay_game_data = game_data.clone();

    // Good learning lesson, this won't work. I need to do an Nvidia overlay hijack.
    let overlay_thread = overlay::thread::spawn_overlay(overlay_config, overlay_game_data);

    let mut menu_config = config.clone();


    let menu_window_name = format!("{} wakey wakey {}",
                                   utils::random_string(4).to_lowercase(),
                                   utils::random_string(4).to_lowercase());

    let menu_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_maximize_button(false)
            .with_title(menu_window_name),
        vsync: true,
        persist_window: true,
        centered: true,
        ..Default::default()
    };

    // Fixed name for state storage
    let menu_storage_name = "wakey wakey";

    // Instead of run_simple_native, use run_native
    eframe::run_native(
        menu_storage_name,
        menu_options,
        Box::new(|cc| {
            Ok(Box::new(crate::gui::menu::Menu::new(config, cc)))
        }),
    ).expect("Unable to create menu window.");




    Ok(())
}