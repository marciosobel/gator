import { invoke } from "@tauri-apps/api/core";

type OpenMainWindowPayload = {
  hideTray: boolean;
};

export async function openMainWindow(
  opts: OpenMainWindowPayload = { hideTray: false },
) {
  console.log("Opening main window...");
  await invoke("open_window", opts);
}
