import { listen, UnlistenFn } from "@tauri-apps/api/event";

export enum CrocEvent {
  HASHING = "croc-hashing",
  SENDING_INFO = "croc-sending-info",
  RECEIVING_INFO = "croc-receiving-info",
  CODE_GENERATED = "croc-code-generated",
  SENDING_TO = "croc-sending-to",
  RECEIVING_FROM = "croc-receiving-from",
  SENDING = "croc-sending",
  RECEIVING = "croc-receiving",
  DONE = "croc-done",
  UNKNOWN = "croc-unknown",
  EOF = "croc-eof",
  INSTANCE_CREATED = "croc-instance-created",
}

export async function listenToCrocEvents(
  handlers: CrocEventHandlers,
): Promise<UnlistenFn[]> {
  return await Promise.all([
    listen<Progress>(CrocEvent.HASHING, (event) =>
      handlers.onHashing?.(event.payload),
    ),
    listen<Progress>(CrocEvent.SENDING, (event) =>
      handlers.onSending?.(event.payload),
    ),
    listen<Progress>(CrocEvent.RECEIVING, (event) =>
      handlers.onReceiving?.(event.payload),
    ),
    listen<Relay>(CrocEvent.RECEIVING_FROM, (event) =>
      handlers.onReceivingFrom?.(event.payload),
    ),
    listen<Relay>(CrocEvent.SENDING_TO, (event) =>
      handlers.onSendingTo?.(event.payload),
    ),
    listen<FileInfo>(CrocEvent.SENDING_INFO, (event) =>
      handlers.onSendingInfo?.(event.payload),
    ),
    listen<FileInfo>(CrocEvent.RECEIVING_INFO, (event) =>
      handlers.onReceivingInfo?.(event.payload),
    ),
    listen<string>(CrocEvent.CODE_GENERATED, (event) =>
      handlers.onCodeGenerated?.(event.payload),
    ),
    listen(CrocEvent.DONE, () => handlers.onDone?.()),
    listen<string>(CrocEvent.UNKNOWN, (event) =>
      handlers.onUnknown?.(event.payload),
    ),
    listen(CrocEvent.EOF, () => handlers.onError?.()),
    listen<number>(CrocEvent.INSTANCE_CREATED, (event) =>
      handlers.onInstanceCreated?.(event.payload),
    ),
  ]);
}

interface CrocEventHandlers {
  onHashing?: (progress: Progress) => MaybePromise<unknown>;
  onSending?: (progress: Progress) => MaybePromise<unknown>;
  onReceiving?: (progress: Progress) => MaybePromise<unknown>;
  onReceivingFrom?: (relay: Relay) => MaybePromise<unknown>;
  onSendingTo?: (relay: Relay) => MaybePromise<unknown>;
  onSendingInfo?: (info: FileInfo) => MaybePromise<unknown>;
  onReceivingInfo?: (info: FileInfo) => MaybePromise<unknown>;
  onCodeGenerated?: (code: string) => MaybePromise<unknown>;
  onDone?: () => MaybePromise<unknown>;
  onUnknown?: (rawLine: string) => MaybePromise<unknown>;
  onError?: () => MaybePromise<unknown>;
  onInstanceCreated?: (pid: number) => MaybePromise<unknown>;
}

export interface Progress {
  fileName: string;
  percentage: number;
  bytesSent?: number;
  bytesTotal?: number;
  speed?: number;
}

export interface FileInfo {
  name: string;
  size: number;
}

export interface Relay {
  address: string;
  port: number;
}
