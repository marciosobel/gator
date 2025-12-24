use tauri::{AppHandle, Emitter};

use crate::{app::Gator, config::tray, croc_sidecar::{CrocCommand, CrocEvent}};

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

    let id = croc.id();
    let mut events = croc.iter().unwrap();
    Gator::insert_croc_instance(&app, croc);

    app.emit("croc-instance-created", id).unwrap();

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
                app.emit("croc-hash-output", data).unwrap()
            }
            CrocEvent::Done => {
                tray::reset_icon(&app);
                if Gator::has_croc_instance(&app, id) {
                    app.emit("croc-done", ()).unwrap();
                }
            }
            CrocEvent::Unknown(raw_line) => app.emit("croc-unknown", raw_line).unwrap(),
            CrocEvent::EOF => app.emit("croc-eof", ()).unwrap(),
        }
    }

    Gator::remove_croc_instance(&app, id);
}
