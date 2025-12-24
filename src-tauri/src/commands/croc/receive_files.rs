use tauri::{AppHandle, Emitter};

use crate::{app::Gator, config::tray, croc_sidecar::{CrocCommand, CrocEvent}};


#[tauri::command]
pub async fn receive_files(app: AppHandle, code: String) {
    let mut command = CrocCommand::new();

    let path = Gator::file_store_path(&app);
    command.no_stdin().no_stdout().pipe_stderr().cd(path);

    let mut croc = command.no_clipboard().yes().receive(&code).spawn().unwrap();

    let id = croc.id();
    let mut events = croc.iter().unwrap();
    Gator::insert_croc_instance(&app, croc);

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
