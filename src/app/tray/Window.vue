<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import Actions from "./Actions.vue";
import PasteCode from "@/components/PasteCode.vue";
import CopyCode from "@/components/CopyCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { Check, LoaderCircle, X } from "lucide-vue-next";
import { onMounted, onUnmounted } from "vue";
import { croc, TransferState } from "@/croc";

onMounted(async () => {
    await croc.subscribe();
});

onUnmounted(() => {
    croc.unsubscribe();
});

const onDrop = async (data: DropzonePayload) => {
    await croc.sendFiles(data.paths);
};

const handleReceive = async (code: string) => {
    await croc.receiveFiles(code);
};

const handleCancel = async () => {
    await croc.kill();
};
</script>

<template>
    <main class="main-wrapper">
        <div class="main-content">
            <template v-if="croc.meta.state == TransferState.NONE">
                <Dropzone @files-dropped="onDrop" />
                <PasteCode @insert="handleReceive" />

                <div v-if="croc.done" class="done-message">
                    <Check :size="16" />
                    <p>File transferred successfully</p>
                </div>
            </template>

            <template v-if="croc.meta.state == TransferState.LOADING">
                <div class="loading">
                    <LoaderCircle class="animate-spin" />
                    <em>Loading, please wait...</em>
                </div>
            </template>

            <template v-if="croc.meta.state == TransferState.WAITING_FOR_CONNECTION">
                <CopyCode :code="croc.meta.code" full-width small-text />
                <span style="text-align: center"> Waiting for someone to connect... </span>
            </template>

            <template v-if="croc.meta.state == TransferState.TRANSFERRING">
                <div class="transferring-title">
                    <span>Transferring {{ croc.meta.progress.fileName }}</span>
                    <span>{{ croc.meta.progress.percentage }}%</span>
                </div>
                <ProgressBar :progress="croc.meta.progress.percentage" />
            </template>

            <template v-if="croc.meta.state == TransferState.HASHING">
                <div class="transferring-title">
                    <span>Hashing {{ croc.meta.progress.fileName }}</span>
                    <span>{{ croc.meta.progress.percentage }}%</span>
                </div>
                <ProgressBar :progress="croc.meta.progress.percentage" />
            </template>

            <button v-if="croc.meta.state != TransferState.NONE" @click="handleCancel" class="cancel-button">
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
