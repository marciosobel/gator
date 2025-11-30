use tauri::App;

pub mod config;

pub type SetupResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> SetupResult {
    #[cfg(desktop)]
    config::setup_tray(app)?;

    Ok(())
}
