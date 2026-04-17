use crate::{config::main_window, utils::window, SetupResult};
use tauri::{
    image::Image,
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, WebviewWindow, Wry,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub const WINDOW_ID: &str = "tray";
const ICON_ID: &str = "1";
const MENU_ITEM_CREATION_ERROR: &str = "unable to create menu item";

/// Setup the system tray icon and its behavior.
pub fn setup(app: &App) -> SetupResult {
    app.handle().plugin(tauri_plugin_positioner::init())?;

    let menu = create_menu(app)?;
    let tray = TrayIconBuilder::new()
        .icon(Icon::Default.into())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_menu_event)
        .build(app)?;

    tray.on_tray_icon_event(handle_tray_event);

    Ok(())
}

fn create_menu(app: &App) -> SetupResult<Menu<Wry>> {
    let quit =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).expect(MENU_ITEM_CREATION_ERROR);
    let toggle_main_window = MenuItem::with_id(
        app,
        "toggle_window_visibility",
        "Show/Hide window",
        true,
        None::<&str>,
    )
    .expect(MENU_ITEM_CREATION_ERROR);

    let menu = Menu::with_items(app, &[&toggle_main_window, &quit])?;

    Ok(menu)
}

fn handle_tray_event(icon: &TrayIcon, event: TrayIconEvent) {
    let handle = icon.app_handle();
    tauri_plugin_positioner::on_tray_event(handle, &event);

    match event {
        TrayIconEvent::Click {
            button,
            button_state,
            ..
        } => handle_click(handle, button, button_state),
        _ => {}
    }
}

fn handle_menu_event(handle: &AppHandle, event: MenuEvent) {
    let event = event.id.as_ref();

    match event {
        "quit" => handle.exit(0),
        "toggle_window_visibility" => toggle_main_window(handle),
        _ => println!("unknown or unhandled tray menu event: {}", event),
    }
}

fn toggle_main_window(handle: &AppHandle) {
    if let Some(window) = main_window::get(handle) {
        window::toggle_visibility(&window);
    } else {
        eprintln!("Unable to find main window to open from tray menu");
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
    handle.get_webview_window(WINDOW_ID)
}

/// Will try to hide the tray window if it exists. No error is returned if it doesn't.
pub fn try_hide(handle: &AppHandle) {
    if let Some(tray) = get_window(handle) {
        window::hide(&tray);
    }
}

/// Gets the current tray.
pub fn get_tray(handle: &AppHandle) -> Option<TrayIcon> {
    if let Some(tray) = handle.tray_by_id(ICON_ID) {
        Some(tray)
    } else {
        eprintln!("Unable to find tray icon by id: {}", ICON_ID);
        None
    }
}

/// Resets the icon to it's default.
pub fn reset_icon(handle: &AppHandle) {
    if let Some(tray) = get_tray(handle) {
        set_icon(&tray, Icon::Default);
    }
}

/// Updates the tray icon.
fn set_icon(tray: &TrayIcon, icon: Icon) {
    if let Err(e) = tray.set_icon(Some(icon.into())) {
        eprintln!("Unable to set tray icon: {}", e);
    }
}

/// Sets the tray icon based on the given progress percentage (between 0 and 100). Default icon will be set otherwise.
pub fn set_icon_progress(handle: &AppHandle, progress: u8) {
    if let Some(tray) = get_tray(handle) {
        let icon = match progress {
            p if p <= 10 => Icon::Progress0,
            p if p <= 20 => Icon::Progress1,
            p if p <= 30 => Icon::Progress2,
            p if p <= 40 => Icon::Progress3,
            p if p <= 50 => Icon::Progress4,
            p if p <= 60 => Icon::Progress5,
            p if p <= 70 => Icon::Progress6,
            p if p <= 80 => Icon::Progress7,
            p if p <= 90 => Icon::Progress8,
            p if p < 100 => Icon::Progress9,
            p if p >= 100 => Icon::Progress10,
            _ => Icon::Default,
        };

        set_icon(&tray, icon);
    } else {
        eprintln!("Unable to find tray icon by id: {}", WINDOW_ID);
        return;
    }
}

const ICON_DEFAULT: Image<'_> = tauri::include_image!("./icons/128x128.png");
const ICON_PROGRESS_0: Image<'_> = tauri::include_image!("./icons/progress/0.png");
const ICON_PROGRESS_1: Image<'_> = tauri::include_image!("./icons/progress/1.png");
const ICON_PROGRESS_2: Image<'_> = tauri::include_image!("./icons/progress/2.png");
const ICON_PROGRESS_3: Image<'_> = tauri::include_image!("./icons/progress/3.png");
const ICON_PROGRESS_4: Image<'_> = tauri::include_image!("./icons/progress/4.png");
const ICON_PROGRESS_5: Image<'_> = tauri::include_image!("./icons/progress/5.png");
const ICON_PROGRESS_6: Image<'_> = tauri::include_image!("./icons/progress/6.png");
const ICON_PROGRESS_7: Image<'_> = tauri::include_image!("./icons/progress/7.png");
const ICON_PROGRESS_8: Image<'_> = tauri::include_image!("./icons/progress/8.png");
const ICON_PROGRESS_9: Image<'_> = tauri::include_image!("./icons/progress/9.png");
const ICON_PROGRESS_10: Image<'_> = tauri::include_image!("./icons/progress/10.png");

pub enum Icon {
    Progress0,
    Progress1,
    Progress2,
    Progress3,
    Progress4,
    Progress5,
    Progress6,
    Progress7,
    Progress8,
    Progress9,
    Progress10,
    Default,
}

impl From<Icon> for Image<'_> {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Progress0 => ICON_PROGRESS_0,
            Icon::Progress1 => ICON_PROGRESS_1,
            Icon::Progress2 => ICON_PROGRESS_2,
            Icon::Progress3 => ICON_PROGRESS_3,
            Icon::Progress4 => ICON_PROGRESS_4,
            Icon::Progress5 => ICON_PROGRESS_5,
            Icon::Progress6 => ICON_PROGRESS_6,
            Icon::Progress7 => ICON_PROGRESS_7,
            Icon::Progress8 => ICON_PROGRESS_8,
            Icon::Progress9 => ICON_PROGRESS_9,
            Icon::Progress10 => ICON_PROGRESS_10,
            Icon::Default => ICON_DEFAULT,
        }
    }
}
