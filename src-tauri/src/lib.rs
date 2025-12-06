use tauri::App;

mod commands;
mod config;
mod utils;

use commands::window::{close_tray, close_window, open_window};

pub type SetupResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            open_window,
            close_window,
            close_tray
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> SetupResult {
    #[cfg(desktop)]
    config::setup_tray(app)?;

    Ok(())
}
