<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import WindowDecorations from "./Decorations.vue";
import Dropzone, { DropzonePayload } from "@/components/Dropzone.vue";
import { UnlistenFn } from "@tauri-apps/api/event";
import { sendFiles, listenToCrocEvents } from "@/events";
import { Clipboard } from "lucide-vue-next";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

const listeners = ref<UnlistenFn[]>([]);
const progress = ref(0);
const code = ref("");
const done = ref(false);

onMounted(async () => {
    listeners.value = await listenToCrocEvents({
        onCodeGenerated: (generated) => {
            code.value = generated;
        },
        onDone: () => {
            done.value = true;
            progress.value = 0;
            code.value = "";
        },
    });
});

onUnmounted(() => {
    listeners.value.forEach((unlisten) => unlisten());
});

const onDrop = async (data: DropzonePayload) => {
    done.value = false;
    await sendFiles(data.paths);
};

const copyCode = async () => {
    await writeText(code.value);
};
</script>

<template>
    <WindowDecorations />
    <main>
        <Dropzone @files-dropped="onDrop" />
        <div class="container">
            <div v-if="code" class="code">
                <p>Code: {{ code }}</p>
                <button @click="copyCode" class="paste-button">
                    <Clipboard :size="16" />
                </button>
            </div>
            <p v-if="progress > 0">Progress: {{ progress }}%</p>
            <div v-if="done">
                <p>Done!</p>
            </div>
        </div>
    </main>
</template>

<style scoped>
.container {
    margin-inline: 20px;
    margin-top: 20px;
}
.code {
    display: flex;
    align-items: center;
    gap: 10px;
}
.paste-button {
    display: flex;
    align-items: center;
    justify-content: center;
    aspect-ratio: 1 / 1;

    border-radius: var(--border-radius);

    border: 1px solid var(--color-border);
}
</style>
