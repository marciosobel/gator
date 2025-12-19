use tauri::{AppHandle, Manager, WebviewWindow};

pub const MAIN_WINDOW_ID: &'static str = "main";

/// Returns the main application window if it exists.
pub fn get(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window(MAIN_WINDOW_ID)
}
