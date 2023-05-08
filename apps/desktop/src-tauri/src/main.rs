#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use inference_server::InferenceServerState;

mod checksum;
mod config;
mod downloader;
mod inference_server;
mod kv_bucket;
mod models_directory;
mod path;

fn main() {
    tauri::Builder::default()
        .manage(InferenceServerState::default())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            downloader::download_model,
            checksum::get_hash,
            checksum::get_cached_hash,
            models_directory::read_directory,
            models_directory::update_models_dir,
            models_directory::initialize_models_dir,
            inference_server::start_server,
            inference_server::stop_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
