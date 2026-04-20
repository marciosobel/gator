use tauri::{App, WindowEvent};

mod app;
mod commands;
mod config;
mod utils;

use commands::{
    croc::{kill_croc_instance, receive_files, send_files},
    window::{close_tray, close_window, open_window},
};

use crate::app::Gator;

pub type SetupResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            open_window,
            close_window,
            close_tray,
            send_files,
            receive_files,
            kill_croc_instance,
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().expect(&format!(
                    "failed to close window with label {}",
                    window.label()
                ));
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> SetupResult {
    #[cfg(desktop)]
    config::setup_tray(app)?;

    Gator::setup(app);

    Ok(())
}
