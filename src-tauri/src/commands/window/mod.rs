use crate::config;

#[tauri::command]
pub fn open_window(handle: tauri::AppHandle, hide_tray: Option<bool>) {
    if let Some(window) = config::window::get_main_window(&handle) {
        config::window::show_main_window(&window);

        let hide_tray = hide_tray.unwrap_or(false);
        if hide_tray {
            config::tray::try_hide_tray(&handle);
        }
    }
}

#[tauri::command]
pub fn close_window(handle: tauri::AppHandle) {
    if let Some(window) = config::window::get_main_window(&handle) {
        config::window::hide_main_window(&window);
    }
}
