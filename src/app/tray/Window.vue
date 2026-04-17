<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import Actions from "./Actions.vue";
import PasteCode from "@/components/PasteCode.vue";
import CopyCode from "@/components/CopyCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { Check, LoaderCircle, X } from "lucide-vue-next";
import {
    listenToCrocEvents,
    Progress,
    receiveFiles,
    sendFiles,
} from "@/events";
import { onMounted, onUnmounted, ref } from "vue";
import { UnlistenFn } from "@tauri-apps/api/event";
import { killCrocInstance } from "@/events/croc/kill-croc-instance";

/** a state representing which point of the file transfer we are */
enum TransferState {
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
}

const listeners = ref<UnlistenFn[]>([]);
const code = ref("");
const transferComplete = ref(false);
const state = ref(TransferState.NONE);
const progress = ref<Progress>();
const crocPID = ref<number>();

onMounted(async () => {
    listeners.value = await listenToCrocEvents({
        onCodeGenerated: (c) => {
            code.value = c;
            state.value = TransferState.WAITING_FOR_CONNECTION;
        },
        onHashing: (newProgress) => {
            progress.value = newProgress;
            transferComplete.value = false;
            state.value = TransferState.HASHING;
        },
        onSending: (newProgress) => {
            progress.value = newProgress;
            state.value = TransferState.TRANSFERRING;
        },
        onDone: () => {
            transferComplete.value = true;
            state.value = TransferState.NONE;
            setTimeout(() => {
                transferComplete.value = false;
            }, 10_000);
        },
        onInstanceCreated(pid) {
            crocPID.value = pid;
        },
    });
});

onUnmounted(() => {
    listeners.value.forEach((unlisten) => unlisten());
});

const onDrop = async (data: DropzonePayload) => {
    state.value = TransferState.LOADING;
    await sendFiles(data.paths);
};

const handleReceive = async (code: string) => {
    state.value = TransferState.LOADING;
    await receiveFiles(code);
};

const handleCancel = async () => {
    state.value = TransferState.LOADING;

    if (crocPID.value !== undefined) {
        await killCrocInstance(crocPID.value);
        crocPID.value = undefined;
    }

    state.value = TransferState.NONE;
};
</script>

<template>
    <main class="main-wrapper">
        <div class="main-content">
            <template v-if="state == TransferState.NONE">
                <Dropzone @files-dropped="onDrop" />
                <PasteCode @insert="handleReceive" />

                <div v-if="transferComplete" class="done-message">
                    <Check :size="16" />
                    <p>File transferred successfully</p>
                </div>
            </template>

            <template v-if="state == TransferState.LOADING">
                <div class="loading">
                    <LoaderCircle class="animate-spin" />
                    <em>Loading, please wait...</em>
                </div>
            </template>

            <template v-if="state == TransferState.WAITING_FOR_CONNECTION">
                <CopyCode :code="code" full-width small-text />
                <span style="text-align: center">
                    Waiting for someone to connect...
                </span>
            </template>

            <template v-if="state == TransferState.TRANSFERRING">
                <div class="transferring-title">
                    <span>Transferring {{ progress!.fileName }}</span>
                    <span>{{ progress!.percentage }}%</span>
                </div>
                <ProgressBar :progress="progress!.percentage" />
            </template>

            <template v-if="state == TransferState.HASHING">
                <div class="transferring-title">
                    <span>Hashing {{ progress!.fileName }}</span>
                    <span>{{ progress!.percentage }}%</span>
                </div>
                <ProgressBar :progress="progress!.percentage" />
            </template>

            <button
                v-if="state != TransferState.NONE"
                @click="handleCancel"
                class="cancel-button"
            >
                <X />Cancel
            </button>
        </div>
        <Actions />
    </main>
</template>

<style scoped>
.main-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin: 5dvw;
    height: 100dvh;
    gap: 10px;
}

.main-content {
    display: flex;
    flex-direction: column;
    gap: 10px;

    height: 100%;
}

.transferring-title {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.done-message {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    margin-inline: auto;
    padding: 12px 12px;

    background: var(--color-success-bg);
    border-radius: var(--border-radius);
}

.done-message p {
    margin: 0;
    font-size: 0.875rem;
}

.loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;

    margin-block: auto;
}

.cancel-button {
    display: flex;
    align-items: center;
    align-self: center;
    gap: 10px;
    width: fit-content;
    cursor: pointer;
}
</style>
