<script setup lang="ts">
import { Event, TauriEvent, UnlistenFn } from "@tauri-apps/api/event";
import {
    DragDropEvent,
    getCurrentWebviewWindow,
} from "@tauri-apps/api/webviewWindow";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload } from "lucide-vue-next";
import { onMounted, onUnmounted, ref } from "vue";

export type DropzonePayload = { paths: string[] };

const emit = defineEmits<{
    (e: "files-dropped", filepaths: DropzonePayload): void;
}>();
const events = ref<UnlistenFn[]>([]);
const hovering = ref(false);

const openDialog = async () => {
    const paths = await open({
        multiple: true,
        title: "Select a file to send",
    });
    if (!paths || paths.length === 0) return;

    emit("files-dropped", { paths });
};

const handleDragDropEvent = (e: Event<DragDropEvent>) => {
    hovering.value = e.event !== TauriEvent.DRAG_LEAVE;

    if (e.event === TauriEvent.DRAG_DROP && e.payload.type === "drop") {
        emit("files-dropped", e.payload);
        hovering.value = false;
    }
};

onMounted(async () => {
    const window = getCurrentWebviewWindow();
    events.value = await Promise.all([
        window.onDragDropEvent(handleDragDropEvent),
    ]);
});
onUnmounted(() => {
    events.value.forEach((unlisten) => unlisten());
});
</script>

<template>
    <div @click="openDialog">
        <span :class="{ hovering }" class="dropzone-area" for="dropzone">
            <Upload :size="24" />
            Select file(s) to upload
        </span>
    </div>
</template>

<style scoped>
.dropzone-area {
    padding: 16px 12px;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    border: 2px dashed var(--color-text);
    border-radius: var(--border-radius);
    opacity: 70%;

    transition: opacity 100ms ease-in-out;
}

.dropzone-area:hover,
.dropzone-area.hovering {
    opacity: 100%;
}
</style>
