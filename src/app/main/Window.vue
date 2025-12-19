<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import WindowDecorations from "./Decorations.vue";
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import CopyCode from "@/components/CopyCode.vue";
import PasteCode from "@/components/PasteCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { UnlistenFn } from "@tauri-apps/api/event";
import { Check } from "lucide-vue-next";
import {
    sendFiles,
    listenToCrocEvents,
    CrocTransferOutput,
    receiveFiles,
    CrocHashOutput,
} from "@/events";

enum TransferState {
    NONE = "none",
    HASHING = "hashing",
    TRANSFERRING = "tranfering",
    WAITING = "waiting",
}

const listeners = ref<UnlistenFn[]>([]);
const generatedCode = ref("");
const done = ref(false);
const state = ref(TransferState.NONE);
const transferOutput = ref<CrocTransferOutput>();
const hashOutput = ref<CrocHashOutput>();

onMounted(async () => {
    listeners.value = await listenToCrocEvents({
        onCodeGenerated: (generated) => {
            done.value = false;
            generatedCode.value = generated;
            state.value = TransferState.WAITING;
        },
        onHashOutput(data) {
            done.value = false;
            hashOutput.value = data;
            state.value = TransferState.HASHING;
        },
        onTransferOutput: (data) => {
            done.value = false;
            transferOutput.value = data;
            state.value = TransferState.TRANSFERRING;
        },
        onDone: () => {
            done.value = true;
            state.value = TransferState.NONE;
            setTimeout(() => {
                done.value = false;
            }, 10_000);
        },
    });
});

onUnmounted(() => {
    listeners.value.forEach((unlisten) => unlisten());
});

const onDrop = async (data: DropzonePayload) => {
    console.log("sending ", data.paths);
    await sendFiles(data.paths);
    console.log("files sent");
};

const onReceive = async (code: string) => {
    await receiveFiles(code);
};
</script>

<template>
    <WindowDecorations />
    <main class="main-content">
        <template v-if="state == TransferState.NONE">
            <Dropzone @files-dropped="onDrop" />
            <PasteCode @receive="onReceive" />
            <div v-if="done" class="done-message">
                <Check />
                <p>File transferred successfully</p>
            </div>
        </template>

        <template v-if="state == TransferState.WAITING">
            <CopyCode :code="generatedCode" />
            <span style="text-align: center"
                >Waiting for someone to connect...</span
            >
        </template>

        <template v-if="state == TransferState.TRANSFERRING">
            <div class="transferring-title">
                <span>Transferring {{ transferOutput!.filename }}</span>
                <span>{{ transferOutput!.progress }}%</span>
            </div>
            <ProgressBar :progress="transferOutput!.progress" />
        </template>

        <template v-if="state == TransferState.HASHING">
            <div class="transferring-title">
                <span>Hashing {{ hashOutput!.filename }}</span>
                <span>{{ hashOutput!.progress }}%</span>
            </div>
            <ProgressBar :progress="hashOutput!.progress" />
        </template>
    </main>
</template>

<style scoped>
.main-content {
    display: flex;
    flex-direction: column;
    margin-block: auto;

    gap: 10px;
    margin-inline: 200px;
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
    flex-direction: column;
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
</style>
