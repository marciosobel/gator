<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import WindowDecorations from "./Decorations.vue";
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import CopyCode from "@/components/CopyCode.vue";
import PasteCode from "@/components/PasteCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { UnlistenFn } from "@tauri-apps/api/event";
import { Check, LoaderCircle, X } from "lucide-vue-next";
import {
    sendFiles,
    listenToCrocEvents,
    CrocTransferOutput,
    receiveFiles,
    CrocHashOutput,
} from "@/events";
import { killCrocInstance } from "@/events/croc/kill-croc-instance";

enum TransferState {
    NONE = "none",
    HASHING = "hashing",
    TRANSFERRING = "tranfering",
    WAITING = "waiting",
    LOADING = "loading",
}

const listeners = ref<UnlistenFn[]>([]);
const generatedCode = ref("");
const isDone = ref(false);
const state = ref(TransferState.NONE);
const transferData = ref<CrocTransferOutput>();
const hashData = ref<CrocHashOutput>();
const locked = ref(true);
const instanceId = ref(0);

onMounted(async () => {
    listeners.value = await listenToCrocEvents({
        onCodeGenerated: (generated) => {
            if (locked.value) return;
            isDone.value = false;
            generatedCode.value = generated;
            state.value = TransferState.WAITING;
        },
        onHashOutput(data) {
            if (locked.value) return;
            isDone.value = false;
            hashData.value = data;
            state.value = TransferState.HASHING;
        },
        onTransferOutput: (data) => {
            if (locked.value) return;
            isDone.value = false;
            transferData.value = data;
            state.value = TransferState.TRANSFERRING;
        },
        onDone: () => {
            if (locked.value) return;
            isDone.value = true;
            state.value = TransferState.NONE;
            setTimeout(() => {
                isDone.value = false;
            }, 10_000);
        },
        onInstanceCreated: (id) => {
            if (locked.value) return;
            instanceId.value = id;
        },
    });
});

onUnmounted(() => {
    listeners.value.forEach((unlisten) => unlisten());
});

const onDrop = async (data: DropzonePayload) => {
    locked.value = false;
    state.value = TransferState.LOADING;
    await sendFiles(data.paths);
};

const handleReceive = async (code: string) => {
    state.value = TransferState.LOADING;
    await receiveFiles(code);
};

const handleCancel = async () => {
    if (instanceId.value === 0) return;
    locked.value = true;
    await killCrocInstance(instanceId.value);
    state.value = TransferState.NONE;
};
</script>

<template>
    <WindowDecorations />
    <main class="main-content">
        <template v-if="state == TransferState.NONE">
            <Dropzone @files-dropped="onDrop" />
            <PasteCode @insert="handleReceive" />
            <div v-if="isDone" class="done-message">
                <Check />
                <p>File transferred successfully</p>
            </div>
        </template>

        <template v-if="state == TransferState.LOADING">
            <div class="loading">
                <LoaderCircle class="animate-spin" />
                <em>Loading, please wait...</em>
            </div>
        </template>

        <template v-if="state == TransferState.WAITING">
            <CopyCode :code="generatedCode" />
            <span style="text-align: center">
                Waiting for someone to connect...
            </span>
        </template>

        <template v-if="state == TransferState.TRANSFERRING">
            <div class="transferring-title">
                <span>Transferring {{ transferData!.filename }}</span>
                <span>{{ transferData!.progress }}%</span>
            </div>
            <ProgressBar :progress="transferData!.progress" />
        </template>

        <template v-if="state == TransferState.HASHING">
            <div class="transferring-title">
                <span>Hashing {{ hashData!.filename }}</span>
                <span>{{ hashData!.progress }}%</span>
            </div>
            <ProgressBar :progress="hashData!.progress" />
        </template>
        <button
            v-if="state != TransferState.NONE"
            @click="handleCancel"
            class="cancel-button"
            :disabled="locked"
        >
            <X />Cancel
        </button>
    </main>
</template>

<style scoped>
.main-content {
    display: flex;
    flex-direction: column;
    margin: auto;

    gap: 10px;
    width: min(90dvw, 400px);
}

.transferring-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.done-message {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    width: fit-content;
    margin-inline: auto;
    padding: 12px 12px;

    background: var(--color-success-bg);
    border-radius: var(--border-radius);
}

.done-message p {
    margin: 0;
}

.loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
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
