import { invoke } from "@tauri-apps/api/core";

export async function closeMainWindow() {
  console.log("Closing main window...");
  await invoke("close_window");
}
