use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use eframe::{egui, EventLoopBuilder, EventLoopBuilderHook};
use crate::config::Config;
use crate::features::GameData;
use crate::gui::overlay::Overlay;
use crate::utils;
use winit::platform::windows::EventLoopBuilderExtWindows;

/// Spawns overlay, returns handle to overlay. Ensure that this thread exits safely.
pub fn spawn_overlay(config: Arc<RwLock<Config>>, game_data: Arc<RwLock<GameData>>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        // Randomize overlay window name
        let window_title = format!("{}", utils::random_string(12).to_lowercase());

        // Get overlay window size
        let window_size = utils::get_varied_window_size(glam::vec2(1920.0, 1080.0), 5.0);

        // Allow this to be run on a separate thread
        let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|builder| {
            builder.with_any_thread(true);
        }));

        let overlay_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_transparent(true)
                .with_always_on_top()
                .with_inner_size([window_size.x, window_size.y])
                .with_fullscreen(true)
                .with_title(window_title),
            vsync: true,
            persist_window: false,
            centered: true,
            event_loop_builder,
            ..Default::default()
        };

        eframe::run_native(
            "overlay",
            overlay_options,
            Box::new(|cc| {
                Ok(Box::new(Overlay::new(config, game_data, cc)))
            }),
        );//.expect("Unable to create overlay window");
    })
}