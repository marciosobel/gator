use tauri::{AppHandle, Emitter};

use crate::{
    config::tray,
    croc_sidecar::{CrocCommand, CrocEvent},
};

#[tauri::command]
pub async fn send_files(app: AppHandle, files: Vec<String>) {
    let mut command = CrocCommand::new();
    let mut croc = command
        .no_stdin()
        .no_stdout()
        .pipe_stderr()
        .no_clipboard()
        .yes()
        .send(&files)
        .spawn()
        .unwrap();

    let mut events = croc.iter().unwrap();
    while let Some(event) = events.next() {
        match event {
            CrocEvent::CodeGenerated(code) => {
                tray::reset_icon(&app);
                app.emit("croc-code-generated", code).unwrap();
            }
            CrocEvent::TransferOutput(data) => {
                tray::set_icon_by_progress(&app, data.progress);
                app.emit("croc-transfer-output", data).unwrap();
            }
            CrocEvent::HashOutput(data) => {
                tray::set_icon_by_progress(&app, data.progress);
                app.emit("croc-hash-output", data).unwrap()},
            CrocEvent::Done => {
                tray::reset_icon(&app);
                app.emit("croc-done", ()).unwrap();
            }
            CrocEvent::Unknown(raw_line) => app.emit("croc-unknown", raw_line).unwrap(),
            CrocEvent::EOF => app.emit("croc-eof", ()).unwrap(),
        }
    }
}

#[tauri::command]
pub async fn receive_files(app: AppHandle, code: String) {
    let mut command = CrocCommand::new();
    let mut croc = command
        .no_stdin()
        .no_stdout()
        .pipe_stderr()
        .no_clipboard()
        .yes()
        .receive(&code)
        .spawn()
        .unwrap();

    let mut events = croc.iter().unwrap();
    while let Some(event) = events.next() {
        match event {
            CrocEvent::CodeGenerated(_) => unreachable!(),
            CrocEvent::TransferOutput(data) => {
                tray::set_icon_by_progress(&app, data.progress);
                app.emit("croc-transfer-output", data).unwrap();
            }
            CrocEvent::HashOutput(data) => app.emit("croc-hash-output", data).unwrap(),
            CrocEvent::Done => {
                tray::reset_icon(&app);
                app.emit("croc-done", ()).unwrap();
            }
            CrocEvent::Unknown(raw_line) => app.emit("croc-unknown", raw_line).unwrap(),
            CrocEvent::EOF => app.emit("croc-eof", ()).unwrap(),
        }
    }
}
