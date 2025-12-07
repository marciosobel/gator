import { invoke } from "@tauri-apps/api/core";

export async function closeTray() {
  console.log("Closing tray...");
  await invoke("close_tray");
}
