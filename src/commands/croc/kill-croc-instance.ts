import { invoke } from "@tauri-apps/api/core";

export async function killCrocInstance(id: number) {
  await invoke("kill_croc_instance", { id });
}
