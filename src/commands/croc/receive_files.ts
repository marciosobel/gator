import { invoke } from "@tauri-apps/api/core";

export async function receiveFiles(code: string) {
  await invoke("receive_files", { code });
}
