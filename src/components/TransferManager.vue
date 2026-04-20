<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import PasteCode from "@/components/PasteCode.vue";
import CopyCode from "@/components/CopyCode.vue";
import ProgressBar from "@/components/ProgressBar.vue";
import { Check, LoaderCircle, X, Eye, EyeOff } from "lucide-vue-next";
import { onMounted, onUnmounted, ref } from "vue";
import { croc, TransferState } from "@/croc";

defineProps<{
    /** Whether the component should render in a compact mode */
    compact?: boolean;
}>();

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

const showRelay = ref(false);
</script>

<template>
    <template v-if="croc.meta.state === TransferState.NONE">
        <Dropzone @files-dropped="onDrop" />
        <PasteCode @insert="handleReceive" />

        <div v-if="croc.done" class="done-message">
            <Check :size="compact ? 16 : 24" />
            <p :style="compact ? 'font-size: 0.875rem;' : ''">File transferred successfully</p>
        </div>
    </template>

    <template v-if="croc.meta.state === TransferState.LOADING">
        <div class="loading">
            <LoaderCircle class="animate-spin" />
            <em>Loading, please wait...</em>
        </div>
    </template>

    <template v-if="croc.meta.state === TransferState.WAITING_FOR_CONNECTION">
        <CopyCode :code="croc.meta.code" :full-width="compact" :small-text="compact" />
        <span style="text-align: center"> Waiting for someone to connect... </span>
    </template>

    <template v-if="croc.meta.state === TransferState.CONNECTING">
        <div class="loading">
            <LoaderCircle class="animate-spin" />
            <em>Connecting...</em>
        </div>
    </template>

    <template v-if="croc.meta.state === TransferState.INFO">
        <div class="loading">
            <LoaderCircle class="animate-spin" />
            <em>Preparing...</em>
        </div>
    </template>

    <template v-if="croc.meta.state === TransferState.TRANSFERRING">
        <div class="transferring-title">
            <div class="state-label">
                <span>{{ croc.mode === "receiving" ? "Receiving" : "Sending" }}</span>
                <span class="filename-inline" :title="croc.meta.progress.file_name">{{
                    croc.meta.progress.file_name
                }}</span>
            </div>
            <span>{{ croc.meta.progress.percentage }}%</span>
        </div>
        <ProgressBar :progress="croc.meta.progress.percentage" />
    </template>

    <template v-if="croc.meta.state === TransferState.HASHING">
        <div class="transferring-title">
            <div class="state-label">
                <span>Hashing</span>
                <span class="filename-inline" :title="croc.meta.progress.file_name">{{
                    croc.meta.progress.file_name
                }}</span>
            </div>
            <span>{{ croc.meta.progress.percentage }}%</span>
        </div>
        <ProgressBar :progress="croc.meta.progress.percentage" />
    </template>

    <div
        v-if="croc.info && croc.meta.state !== TransferState.TRANSFERRING && croc.meta.state !== TransferState.HASHING"
        style="text-align: center"
    >
        <span class="filename-subtitle">{{ croc.info.name }}</span>
    </div>

    <div v-if="croc.relay" class="relay-info">
        <span class="label">{{ croc.mode === "sending" ? "Sending to" : "Receiving from" }}:</span>
        <span class="value" v-if="showRelay">{{ croc.relay.address }}:{{ croc.relay.port }}</span>
        <span class="value" v-else>••••••••••••</span>
        <button @click="showRelay = !showRelay" class="icon-btn" title="Toggle relay visibility">
            <Eye v-if="!showRelay" :size="12" />
            <EyeOff v-else :size="12" />
        </button>
    </div>

    <button v-if="croc.meta.state !== TransferState.NONE" @click="handleCancel" class="cancel-button">
        <X />Cancel
    </button>
</template>

<style scoped>
.transferring-title {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
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

.filename-subtitle {
    display: block;
    font-size: 0.8rem;
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: center;
}

.state-label {
    display: flex;
    align-items: baseline;
    gap: 6px;
    overflow: hidden;
    white-space: nowrap;
    min-width: 0;
    flex: 1;
}

.filename-inline {
    font-size: 0.85rem;
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
}

.relay-info {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    font-size: 0.75rem;
    opacity: 0.8;
}

.label {
    font-weight: 600;
    opacity: 0.7;
}

.value {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.icon-btn {
    background: transparent;
    border: none;
    padding: 4px;
    border-radius: var(--border-radius);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: inherit;
    opacity: 0.7;
    transition:
        background 0.2s,
        opacity 0.2s;
}

.icon-btn:hover {
    opacity: 1;
    background: var(--color-background-mute, rgba(128, 128, 128, 0.2));
}
</style>
