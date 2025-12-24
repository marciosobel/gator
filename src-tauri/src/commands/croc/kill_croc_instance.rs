use tauri::AppHandle;

use crate::app::Gator;

#[tauri::command]
pub fn kill_croc_instance(app: AppHandle, id: u32) {
    let state = Gator::get_state(&app);
    let mut state = state.lock().expect("Failed to lock app state");
    if let Some(child) = state.croc_instances.get_mut(&id) {
        child.kill().expect("Failed to kill croc instance");
        state.croc_instances.remove(&id);
    }
}
