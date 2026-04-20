import type { Progress, FileInfo, Relay } from "@/commands";

/** a state representing which point of the file transfer we are */
export enum TransferState {
  /** app is in idle state waiting for user to choose between receiving a file or sending one */
  NONE = "none",
  /** `croc` is hashing the file to send it */
  HASHING = "hashing",
  /** a transference is currently active */
  TRANSFERRING = "tranfering",
  /** app is waiting for someone to connect in order to send files */
  WAITING_FOR_CONNECTION = "waiting",
  /** the app is waiting for the croc instance to be created */
  LOADING = "loading",
  /** `croc` is connecting to a relay */
  CONNECTING = "connecting",
  /** `croc` is exchanging file info */
  INFO = "info",
}

export type TransferMetadata =
  | {
      state: TransferState.NONE | TransferState.LOADING;
    }
  | {
      state: TransferState.HASHING | TransferState.TRANSFERRING;
      progress: Progress;
    }
  | {
      state: TransferState.WAITING_FOR_CONNECTION;
      code: string;
    }
  | {
      state: TransferState.CONNECTING;
      relay: Relay;
    }
  | {
      state: TransferState.INFO;
      info: FileInfo;
    };
