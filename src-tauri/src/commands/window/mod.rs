use crate::{
    config::{main_window, tray},
    utils::window,
};

#[tauri::command]
pub fn open_window(handle: tauri::AppHandle, hide_tray: Option<bool>) {
    if let Some(window) = main_window::get(&handle) {
        window::show_and_focus(&window);

        let hide_tray = hide_tray.unwrap_or(false);
        if hide_tray {
            tray::try_hide(&handle);
        }
    }
}

#[tauri::command]
pub fn close_window(handle: tauri::AppHandle) {
    if let Some(window) = main_window::get(&handle) {
        window::hide(&window);
    }
}
