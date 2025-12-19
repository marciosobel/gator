<script setup lang="ts">
import { readText } from "@tauri-apps/plugin-clipboard-manager";
import { ArrowRight, Clipboard, Lightbulb } from "lucide-vue-next";
import { ref } from "vue";

const props = defineProps<{
    onInsert: (code: string) => void | Promise<void>;
}>();

const insertedCode = ref("");

const pasteCode = async () => {
    try {
        const code = await readText();
        insertedCode.value = code.trim();
    } catch (e) {
        console.error("[ERROR] Failed to paste clipboard contents: ", e);
    }
};
</script>

<template>
    <form
        class="insert-code-container"
        @submit.prevent="props.onInsert(insertedCode)"
    >
        <div class="input-container">
            <input
                v-model="insertedCode"
                placeholder="Insert code"
                class="insert-code-input"
                name="insert-code-input"
            />
            <button type="button" class="paste-button" @click="pasteCode">
                <Clipboard :size="20" />
            </button>
        </div>

        <div class="start-receiving-container">
            <span class="start-receiving-hint">
                <Lightbulb :size="16" />
                You can press Enter instead!
            </span>

            <button
                :disabled="!insertedCode"
                class="start-receiving-button"
                type="submit"
            >
                Go
                <ArrowRight :size="16" />
            </button>
        </div>
    </form>
</template>

<style scoped>
.insert-code-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 10px;
}

.input-container {
    display: flex;

    height: 32px;
    background: var(--color-bg-weaker);
    border: 1px solid var(--color-border);
    border-radius: var(--border-radius);
}

.input-container:focus-within {
    border: 1px solid var(--color-text);
}

.insert-code-input {
    width: 100%;
    padding-left: 10px;
    background: var(--color-bg-weaker);
    border-radius: var(--border-radius);
}

.insert-code-input:focus {
    outline: none;
}

.paste-button {
    display: flex;
    align-items: center;
    justify-content: center;

    height: 100%;
    aspect-ratio: 1 / 1;

    border-radius: var(--border-radius);
}

.paste-button:hover,
.start-receiving-button:hover {
    background: var(--color-bg-weak);
}

.paste-button:active,
.start-receiving-button:active {
    background: var(--color-bg-active);
}

.paste-button:disabled,
.start-receiving-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background: var(--color-bg-weaker);
}

.start-receiving-container {
    display: flex;
    justify-content: space-between;
    align-items: start;
}

.start-receiving-button {
    display: flex;
    align-items: center;
    gap: 5px;

    padding: 4px 10px;

    border-radius: var(--border-radius);
    transition:
        background 100ms ease,
        opacity 150ms ease;
}

.start-receiving-hint {
    display: flex;
    align-items: center;
    gap: 5px;

    opacity: 70%;
    font-size: 0.675rem;
}
</style>
