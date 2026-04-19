import { invoke } from "@tauri-apps/api/core";

export async function sendFiles(files: string[]) {
  await invoke("send_files", { files });
}
