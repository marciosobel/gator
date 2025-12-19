use tauri::WebviewWindow;

/// Toggles the visibility of the window. Returns true on success.
pub fn toggle_visibility(window: &WebviewWindow) -> bool {
    if is_visible(window) {
        return hide(window);
    } else {
        return show(window);
    }
}

/// Checks if the window is visible. Returns true if visible, false otherwise.
pub fn is_visible(window: &WebviewWindow) -> bool {
    match window.is_visible() {
        Ok(v) => v,
        Err(e) => {
            log_err("Failed to check window visibility", window, e);
            false
        }
    }
}

/// Shows the window. Returns true on success.
pub fn show(window: &WebviewWindow) -> bool {
    if let Err(e) = window.show() {
        log_err("Failed to show existing window", window, e);
        return false;
    }

    true
}

/// Focuses the window. Returns true on success.
pub fn focus(window: &WebviewWindow) -> bool {
    if let Err(e) = window.set_focus() {
        log_err("Failed to focus existing window", window, e);
        return false;
    }
    true
}

/// Shows and focuses the window. Returns true on success.
pub fn show_and_focus(window: &WebviewWindow) -> bool {
    if !show(window) || !focus(window) {
        return false;
    }

    true
}

/// Hides the window. Returns true on success.
pub fn hide(window: &WebviewWindow) -> bool {
    if let Err(e) = window.hide() {
        log_err("Failed to hide existing window", window, e);
        return false;
    }

    true
}

fn log_err(msg: &str, window: &WebviewWindow, err: impl std::error::Error) {
    eprintln!("Window error: ({}) {}: {}", window.label(), msg, err);
}
