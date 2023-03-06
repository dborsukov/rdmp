#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
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
            cmd::load_roadmap,
            cmd::load_all_roadmaps,
            cmd::add_roadmap,
            cmd::update_roadmap,
            cmd::remove_roadmap,
            cmd::add_node,
            cmd::update_node,
            cmd::remove_node,
            cmd::set_done,
            cmd::set_skip,
            cmd::load_details,
            cmd::save_details,
            cmd::load_roadmaps_amount,
            cmd::load_nodes_amount,
            cmd::read_settings,
            cmd::write_settings,
            cmd::export_roadmap,
            cmd::import_roadmap,
            cmd::expand_nodes_around,
            cmd::squash_nodes_around,
        ])
        .run(tauri::generate_context!())
    {
        error!("Failed to start Tauri application: {err}");
        process::exit(1);
    }
}
