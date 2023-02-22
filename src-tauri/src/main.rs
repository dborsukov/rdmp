#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use log::error;
use std::process;

pub mod cmd;
pub mod db;
pub mod fs;
pub mod logger;
pub mod models;
pub mod schema;

fn main() {
    fs::validate();
    logger::setup();
    db::run_migrations();

    if let Err(err) = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::load_all_roadmaps,
            cmd::add_roadmap,
            cmd::update_roadmap,
            cmd::remove_roadmap,
        ])
        .run(tauri::generate_context!())
    {
        error!("Failed to start Tauri application: {err}");
        process::exit(1);
    }
}
