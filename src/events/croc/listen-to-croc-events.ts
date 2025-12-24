import { listen, UnlistenFn } from "@tauri-apps/api/event";

export enum CrocEvent {
  TransferOutput = "croc-transfer-output",
  HashOutput = "croc-hash-output",
  CodeGenerated = "croc-code-generated",
  Done = "croc-done",
  Unknown = "croc-unknown",
  Error = "croc-error",
  InstanceCreated = "croc-instance-created",
}

export async function listenToCrocEvents(
  handlers: CrocEventHandlers,
): Promise<UnlistenFn[]> {
  return await Promise.all([
    listen<CrocTransferOutput>(CrocEvent.TransferOutput, (event) =>
      handlers.onTransferOutput?.(event.payload),
    ),
    listen<CrocHashOutput>(CrocEvent.HashOutput, (event) =>
      handlers.onHashOutput?.(event.payload),
    ),
    listen<string>(CrocEvent.CodeGenerated, (event) =>
      handlers.onCodeGenerated?.(event.payload),
    ),
    listen<void>(CrocEvent.Done, () => handlers.onDone?.()),
    listen<string>(CrocEvent.Unknown, (event) =>
      handlers.onUnknown?.(event.payload),
    ),
    listen<void>(CrocEvent.Error, () => handlers.onError?.()),
    listen<number>(CrocEvent.InstanceCreated, (event) =>
      handlers.onInstanceCreated?.(event.payload),
    ),
  ]);
}

interface CrocEventHandlers {
  onTransferOutput?: (data: CrocTransferOutput) => MaybePromise<unknown>;
  onHashOutput?: (data: CrocHashOutput) => MaybePromise<unknown>;
  onCodeGenerated?: (code: string) => MaybePromise<unknown>;
  onDone?: () => MaybePromise<unknown>;
  onUnknown?: (rawLine: string) => MaybePromise<unknown>;
  onError?: () => MaybePromise<unknown>;
  onInstanceCreated?: (id: number) => MaybePromise<unknown>;
}

export interface CrocTransferOutput {
  progress: number;
  total_size: number;
  total_sent: number;
  speed: number;
  time_spent?: number;
  time_remaining?: number;
  filename: string;
  raw_message: string;
}

export interface CrocHashOutput {
  progress: number;
  speed: number;
  time_spent: number;
  time_remaining: number;
  filename: string;
  raw_message: string;
}
