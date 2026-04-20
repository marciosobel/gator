import { UnlistenFn } from "@tauri-apps/api/event";
import { TransferMetadata, TransferState } from "./transfer-state";
import { killCrocInstance, listenToCrocEvents, receiveFiles, sendFiles, type FileInfo, type Relay } from "@/commands";

export class Croc {
  private _meta: TransferMetadata = { state: TransferState.NONE };
  private listeners: UnlistenFn[] = [];
  private _done: boolean = false;
  private _pid: number | null = null;
  private _info: FileInfo | null = null;
  private _relay: Relay | null = null;
  private _mode: "sending" | "receiving" | null = null;

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
      onSendingTo: (relay) => {
        this.relay = relay;
        this.meta = {
          state: TransferState.CONNECTING,
          relay,
        };
      },
      onReceivingFrom: (relay) => {
        this.relay = relay;
        this.meta = {
          state: TransferState.CONNECTING,
          relay,
        };
      },
      onSendingInfo: (info) => {
        this.info = info;
        this.meta = {
          state: TransferState.INFO,
          info,
        };
      },
      onReceivingInfo: (info) => {
        this.info = info;
        this.meta = {
          state: TransferState.INFO,
          info,
        };
      },
      onDone: () => {
        this.done = true;
        this.info = null;
        this.relay = null;
        this.mode = null;
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
    if (this.pid !== null) await this.kill();

    this.mode = "sending";
    this.meta.state = TransferState.LOADING;
    await sendFiles(filepaths);
  }

  /** Receive files through croc */
  async receiveFiles(code: string) {
    if (this.pid !== null) await this.kill();

    this.mode = "receiving";
    this.meta.state = TransferState.LOADING;
    await receiveFiles(code);
  }

  /** Kills the running croc instance, if it exists. If killed, state is changed to NONE. */
  async kill() {
    if (this.pid === null) return;

    this.meta.state = TransferState.LOADING;
    await killCrocInstance(this.pid);
    this.pid = null;
    this.info = null;
    this.relay = null;
    this.mode = null;

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

  get info(): FileInfo | null {
    return this._info;
  }

  private set info(value: FileInfo | null) {
    this._info = value;
  }

  get relay(): Relay | null {
    return this._relay;
  }

  private set relay(value: Relay | null) {
    this._relay = value;
  }

  get mode(): "sending" | "receiving" | null {
    return this._mode;
  }

  private set mode(value: "sending" | "receiving" | null) {
    this._mode = value;
  }
}
