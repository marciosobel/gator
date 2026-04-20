<script lang="ts" setup>
import { debounce } from "@/utils/debounce";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Copy, Check } from "lucide-vue-next";
import { ref } from "vue";

const copied = ref(false);
const props = defineProps<{
    code: string;
    fullWidth?: boolean;
    smallText?: boolean;
}>();

const uncopy = debounce(() => {
    copied.value = false;
}, 2_000);
const copyCode = async () => {
    await writeText(props.code);
    copied.value = true;
    uncopy();
};
</script>

<template>
    <div class="copy-code-container">
        <span class="code-message">Code generated</span>
        <div class="copy-code-box" :class="{ copied, fullWidth }">
            <span v-if="copied" class="copy-code-copied-message">Copied!</span>
            <span :class="{ copied, fullWidth, smallText }" class="copy-code-code">{{ props.code }}</span>
            <button :class="{ copied }" class="copy-code-button" @click="copyCode">
                <Check v-if="copied" :size="20" />
                <Copy v-else :size="20" />
            </button>
        </div>
    </div>
</template>

<style scoped>
.copy-code-container {
    display: flex;
    flex-direction: column;
    align-items: center;

    gap: 5px;
}

.code-message {
    font-size: 0.875rem;
    opacity: 60%;
}

.copy-code-code,
.copy-code-copied-message {
    font-size: 1.25rem;
}

.copy-code-copied-message {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    user-select: none;
}

.copy-code-code {
    letter-spacing: 3px;
    text-wrap: nowrap;
    padding-left: 5px;
}

.copy-code-code.smallText {
    font-size: 0.875rem;
    text-wrap: wrap;
}

.copy-code-code.copied {
    opacity: 0;
    user-select: none;
}

.copy-code-box {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    background: var(--color-bg-weaker);
    border-radius: var(--border-radius);
    padding: 5px;
    padding-left: 10px;
}

.copy-code-box.fullWidth {
    width: 100%;
}

.copy-code-box.copied {
    background: var(--color-success-bg);
}

.copy-code-button {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: auto;

    aspect-ratio: 1 / 1;
    border-radius: var(--border-radius);
    cursor: pointer;
    background: transparent;
}

.copy-code-button:hover {
    background: rgba(255, 255, 255, 0.05);
}

.copy-code-button:active {
    background: rgba(255, 255, 255, 0.1);
}

.copy-code-copied-message,
.copy-code-button.copied * {
    color: var(--color-success);
}
</style>
