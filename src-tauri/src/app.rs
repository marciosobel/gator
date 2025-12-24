use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use tauri::{App, AppHandle, Manager};

use crate::croc_sidecar::CrocChild;

pub struct Gator {
    pub croc_instances: HashMap<u32, CrocChild>,
}

impl Gator {
    pub fn new() -> Self {
        Self {
            croc_instances: HashMap::new(),
        }
    }

    /// Sets up app to use this struct as its state
    pub fn setup(app: &App) {
        app.manage(Mutex::new(Self::new()));
    }

    /// Returns `Self` from the current app state.
    /// If using in the middle of a function, remember to
    /// call `drop` in order to unlock the inner mutex.
    pub fn get_state(handle: &AppHandle) -> tauri::State<'_, Mutex<Self>> {
        handle.state::<Mutex<Self>>()
    }

    /// Inserts a croc instance to the hashmap.
    pub fn insert_croc_instance(handle: &AppHandle, child: CrocChild) {
        let id = child.id();
        let state = Gator::get_state(&handle);
        let mut state = state.lock().expect("Failed to lock app mutex");
        state.croc_instances.insert(id, child);
    }

    pub fn has_croc_instance(handle: &AppHandle, id: u32) -> bool {
        let state = Gator::get_state(&handle);
        let state = state.lock().expect("Failed to lock app mutex");
        state.croc_instances.contains_key(&id)
    }

    /// Removes a croc instance from the hashmap.
    pub fn remove_croc_instance(handle: &AppHandle, id: u32) {
        let state = Gator::get_state(&handle);
        let mut state = state.lock().expect("Failed to lock app mutex");
        state.croc_instances.remove(&id);
    }

    /// Returns the default directory to save files
    pub fn file_store_path(handle: &AppHandle) -> PathBuf {
        let mut path = handle
            .path()
            .document_dir()
            .expect("Failed to get document directory");

        path.push("Gator");
        path.push("received_files");
        if !path.exists() {
            std::fs::create_dir_all(path.clone()).expect("Failed to create directories");
        }

        path
    }
}

impl Default for Gator {
    fn default() -> Self {
        Gator::new()
    }
}
