use croc_sidecar::{Croc, CrocEvent};
use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};

use crate::{app::Gator, config::tray};

#[tauri::command]
pub async fn send_files(app: AppHandle, files: Vec<String>) {
    let mut croc = Croc::new()
        .clipboard(false)
        .send()
        .files(files)
        .spawn()
        .expect("Failed to spawn croc instance");

    let id = croc.id().expect("Failed to get `croc` PID");
    let mut events = croc.events().expect("Failed to create croc event stream");

    Gator::insert_croc_instance(&app, croc);
    app.emit("croc-instance-created", id).unwrap();

    while let Some(event) = events.next().await {
        match event {
            CrocEvent::CodeGenerated(code) => {
                tray::reset_icon(&app);
                app.emit("croc-code-generated", code).unwrap();
            }
            CrocEvent::Hashing(progress) => {
                tray::set_icon_progress(&app, progress.percentage);
                app.emit("croc-hashing", progress).unwrap();
            }
            CrocEvent::SendingInfo(info) => app.emit("croc-sending-info", info).unwrap(),
            CrocEvent::SendingTo(relay) => app.emit("croc-sending-to", relay).unwrap(),
            CrocEvent::Sending(progress) => {
                tray::set_icon_progress(&app, progress.percentage);
                app.emit("croc-sending", progress).unwrap();
            }
            CrocEvent::Done => {
                tray::reset_icon(&app);
                if Gator::has_croc_instance(&app, id) {
                    app.emit("croc-done", ()).unwrap();
                }
            }
            CrocEvent::Unknown(raw) => app.emit("croc-unknown", raw).unwrap(),
            CrocEvent::EOF => app.emit("croc-eof", ()).unwrap(),
            _ => unreachable!("Received event that should only be received in `receive` context."),
        }
    }

    Gator::remove_croc_instance(&app, id);
}
