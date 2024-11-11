#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod utils;
mod gui;
mod config;
mod features;
mod view;

use eframe::egui;
use crate::config::Config;
use crate::gui::menu::Menu;

fn main() -> eframe::Result {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp_secs()
        .init();

    // Randomize app name
    let window_name = format!("{} wakey wakey {}",
                              utils::random_string(4).to_lowercase()
                              , utils::random_string(4).to_lowercase());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_title(window_name),
        vsync: true,
        run_and_return: false,
        persist_window: true,
        centered: true,
        ..Default::default()
    };

    // Load config from file or create default
    let mut config = Config::load().unwrap_or_else(|e| {
        log::error!("Failed to load config: {}, using default", e);
        Config::default()
    });

    // Fixed name for state storage
    let storage_name = "wakey wakey";

    // Instead of run_simple_native, use run_native
    eframe::run_native(
        storage_name,
        options,
        Box::new(|cc| {
            Ok(Box::new(Menu::new(&mut config, cc)))
        }),
    )
}