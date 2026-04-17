use croc_sidecar::{Croc, CrocEvent};
use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};

use crate::{app::Gator, config::tray};

#[tauri::command]
pub async fn receive_files(app: AppHandle, code: String) {
    let path = Gator::default_receive_filepath(&app);
    let mut croc = Croc::new()
        .receive()
        .out(path)
        .spawn(code)
        .expect("Failed to spawn croc instance");

    let id = croc.id().expect("Failed to get `croc` PID");
    let mut events = croc.events().expect("Failed to create croc event stream");

    Gator::insert_croc_instance(&app, croc);
    app.emit("croc-instance-created", id).unwrap();

    while let Some(event) = events.next().await {
        match event {
            CrocEvent::ReceivingInfo(info) => app.emit("croc-receiving-info", info).unwrap(),
            CrocEvent::ReceivingFrom(relay) => app.emit("croc-receiving-from", relay).unwrap(),
            CrocEvent::Receiving(progress) => {
                tray::set_icon_progress(&app, progress.percentage);
                app.emit("croc-transfer-output", progress).unwrap();
            }
            CrocEvent::Done => {
                tray::reset_icon(&app);
                if Gator::has_croc_instance(&app, id) {
                    app.emit("croc-done", ()).unwrap();
                }
            }
            CrocEvent::Unknown(raw) => app.emit("croc-unknown", raw).unwrap(),
            CrocEvent::EOF => app.emit("croc-eof", ()).unwrap(),
            _ => unreachable!("Received event that should only be received in `send` context."),
        }
    }

    Gator::remove_croc_instance(&app, id);
}
