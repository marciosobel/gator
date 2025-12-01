use tauri::{AppHandle, Manager, WebviewWindow};

pub const MAIN_WINDOW_ID: &'static str = "main";

pub fn toggle_main_window_visibility(window: &WebviewWindow) {
    let visible = match window.is_visible() {
        Err(e) => {
            eprintln!("Failed to get main window visibility: {}", e);
            return;
        }
        Ok(v) => v,
    };

    if visible {
        show_main_window(window);
    } else {
        hide_main_window(window);
    }
}

pub fn get_main_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window(MAIN_WINDOW_ID)
}

pub fn show_main_window(window: &WebviewWindow) {
    if let Err(e) = window.show() {
        eprintln!("Failed to show existing main window: {}", e);
    }

    if let Err(e) = window.set_focus() {
        eprintln!("Failed to focus existing main window: {}", e);
    }
}

pub fn hide_main_window(window: &WebviewWindow) {
    if let Err(e) = window.hide() {
        eprintln!("Failed to hide main window: {}", e);
    }
}
