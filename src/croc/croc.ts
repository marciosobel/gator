import { UnlistenFn } from "@tauri-apps/api/event";
import { TransferMetadata, TransferState } from "./transfer-state";
import { killCrocInstance, listenToCrocEvents, receiveFiles, sendFiles } from "@/commands";

export class Croc {
  private _meta: TransferMetadata = { state: TransferState.NONE };
  private listeners: UnlistenFn[] = [];
  private _done: boolean = false;
  private _pid: number | null = null;

  /** Subscribes to all croc events. */
  async subscribe() {
    this.listeners = await listenToCrocEvents({
      onCodeGenerated: (code) => {
        this.meta = {
          state: TransferState.WAITING_FOR_CONNECTION,
          code,
        };
      },
      onHashing: (progress) => {
        this.done = false;
        this.meta = {
          state: TransferState.HASHING,
          progress,
        };
      },
      onSending: (progress) => {
        this.done = false;
        this.meta = {
          state: TransferState.TRANSFERRING,
          progress,
        };
      },
      onReceiving: (progress) => {
        this.done = false;
        this.meta = {
          state: TransferState.TRANSFERRING,
          progress,
        };
      },
      onDone: () => {
        this.done = true;
        this.meta.state = TransferState.NONE;
        setTimeout(() => {
          this.done = false;
        }, 10_000);
      },
      onInstanceCreated: (pid) => {
        this.pid = pid;
      },
    });
  }

  /** Unlistens to all croc event listeners. */
  unsubscribe() {
    this.listeners.forEach((f) => f());
  }

  /** Send files through croc */
  async sendFiles(filepaths: string[]) {
    this.meta.state = TransferState.LOADING;
    await sendFiles(filepaths);
  }

  /** Receive files through croc */
  async receiveFiles(code: string) {
    this.meta.state = TransferState.LOADING;
    await receiveFiles(code);
  }

  /** Kills the running croc instance, if it exists. If killed, state is changed to NONE. */
  async kill() {
    if (this.pid === null) return;

    this.meta.state = TransferState.LOADING;
    await killCrocInstance(this.pid);
    this.pid = null;

    this.meta.state = TransferState.NONE;
  }

  get meta(): TransferMetadata {
    return this._meta;
  }

  private set meta(value: TransferMetadata) {
    this._meta = value;
  }

  get done(): boolean {
    return this._done;
  }

  private set done(value: boolean) {
    this._done = value;
  }

  get pid(): number | null {
    return this._pid;
  }

  private set pid(value: number | null) {
    this._pid = value;
  }
}
