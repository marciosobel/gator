use crate::{utils::window, SetupResult};
use tauri::{
    image::Image,
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, WebviewWindow, Wry,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub const TRAY_WINDOW_ID: &'static str = "tray";

/// Setup the system tray icon and its behavior.
pub fn setup(app: &App) -> SetupResult {
    app.handle().plugin(tauri_plugin_positioner::init())?;

    let menu = tray_menu(app)?;
    let icon = tray_icon(app);
    let tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_menu_event)
        .build(app)?;

    tray.on_tray_icon_event(handle_tray_event);

    Ok(())
}

static MENU_ITEM_CREATION_ERROR: &'static str = "unable to create menu item";

fn tray_menu(app: &App) -> SetupResult<Menu<Wry>> {
    let quit =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).expect(MENU_ITEM_CREATION_ERROR);
    let open_main_window = MenuItem::with_id(
        app,
        "open_main_window",
        "Open main window",
        false,
        None::<&str>,
    )
    .expect(MENU_ITEM_CREATION_ERROR);

    let menu = Menu::with_items(app, &[&open_main_window, &quit])?;
    Ok(menu)
}

fn tray_icon(app: &App) -> Image<'_> {
    app.default_window_icon()
        .expect("should be able to get default window icon")
        .clone()
}

fn handle_tray_event(icon: &TrayIcon, event: TrayIconEvent) {
    let app = icon.app_handle();
    tauri_plugin_positioner::on_tray_event(app, &event);

    match event {
        TrayIconEvent::Click {
            button,
            button_state,
            ..
        } => handle_click(app, button, button_state),
        _ => {}
    }
}

fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    let event = event.id.as_ref();

    match event {
        "quit" => app.exit(0),
        _ => println!("unknown tray menu event: {}", event),
    }
}

fn handle_click(handle: &AppHandle, button: MouseButton, button_state: MouseButtonState) {
    match get_window(handle) {
        Some(window) => {
            // move window, no matter the button or the button state
            if let Err(e) = window.move_window(Position::TrayCenter) {
                eprintln!("Unable to position tray window: {}", e);
            }

            if button == MouseButton::Left && button_state == MouseButtonState::Up {
                window::toggle_visibility(&window);
            }
        }
        None => {
            eprintln!("Unable to find tray window on tray icon click",);
        }
    }
}

/// Returns the tray window if it exists.
pub fn get_window(handle: &AppHandle) -> Option<WebviewWindow> {
    handle.get_webview_window(TRAY_WINDOW_ID)
}

/// Will try to hide the tray window if it exists. No error is returned if it doesn't.
pub fn try_hide(handle: &AppHandle) {
    if let Some(tray) = get_window(handle) {
        window::hide(&tray);
    }
}
