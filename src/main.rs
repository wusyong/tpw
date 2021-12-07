#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

fn main() {
    let app = tpw::PasswordWindow::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2 { x: 400., y: 120. }),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
