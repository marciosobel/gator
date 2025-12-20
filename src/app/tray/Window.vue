<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import Actions from "./Actions.vue";
import PasteCode from "@/components/PasteCode.vue";
import CopyCode from "@/components/CopyCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { Check, LoaderCircle } from "lucide-vue-next";
import {
    CrocHashOutput,
    CrocTransferOutput,
    listenToCrocEvents,
    receiveFiles,
    sendFiles,
} from "@/events";
import { onMounted, onUnmounted, ref } from "vue";
import { UnlistenFn } from "@tauri-apps/api/event";

enum TransferState {
    NONE = "none",
    HASHING = "hashing",
    TRANSFERRING = "tranfering",
    WAITING = "waiting",
    LOADING = "loading",
}

const listeners = ref<UnlistenFn[]>([]);
const state = ref<TransferState>(TransferState.NONE);
const generatedCode = ref("");
const transferData = ref<CrocTransferOutput>();
const hashData = ref<CrocHashOutput>();
const isDone = ref(false);

const onDrop = async (data: DropzonePayload) => {
    state.value = TransferState.LOADING;
    await sendFiles(data.paths);
};

const handleReceive = async (code: string) => {
    state.value = TransferState.LOADING;
    await receiveFiles(code);
};

onMounted(async () => {
    listeners.value = await listenToCrocEvents({
        onCodeGenerated: (generated) => {
            isDone.value = false;
            generatedCode.value = generated;
            state.value = TransferState.WAITING;
        },
        onHashOutput(data) {
            isDone.value = false;
            hashData.value = data;
            state.value = TransferState.HASHING;
        },
        onTransferOutput: (data) => {
            isDone.value = false;
            transferData.value = data;
            state.value = TransferState.TRANSFERRING;
        },
        onDone: () => {
            isDone.value = true;
            state.value = TransferState.NONE;
            setTimeout(() => {
                isDone.value = false;
            }, 5_000);
        },
    });
});

onUnmounted(() => {
    listeners.value.forEach((unlisten) => unlisten());
});
</script>

<template>
    <main class="main-wrapper">
        <div class="main-content">
            <template v-if="state == TransferState.NONE">
                <Dropzone @files-dropped="onDrop" />
                <PasteCode @insert="handleReceive" />

                <div v-if="isDone" class="done-message">
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

            <template v-if="state == TransferState.WAITING">
                <CopyCode :code="generatedCode" full-width small-text />
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

.animate-spin {
    animation: spin 1.5s linear infinite;
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}
</style>
