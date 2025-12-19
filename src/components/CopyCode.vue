<script lang="ts" setup>
import { debounce } from "@/utils/debounce";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Copy, Check } from "lucide-vue-next";
import { ref } from "vue";

const copied = ref(false);
const props = defineProps<{
    code: string;
}>();

const uncopy = debounce(() => {
    copied.value = false;
}, 1000);
const copyCode = async () => {
    await writeText(props.code);
    copied.value = true;
    uncopy();
};
</script>

<template>
    <div class="copy-code-container">
        <span class="code-message">Code generated</span>
        <div class="copy-code-box" :class="{ copied }">
            <span v-if="copied" class="copy-code-copied">Copied!</span>
            <span :class="{ copied }" class="copy-code-code">{{
                props.code
            }}</span>
            <button
                :class="{ copied }"
                class="copy-code-button"
                @click="copyCode"
            >
                <Check v-if="copied" />
                <Copy v-else />
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
.copy-code-copied {
    font-size: 1.25rem;
}

.copy-code-copied {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
}

.copy-code-code {
    letter-spacing: 3px;
    text-wrap: nowrap;
}

.copy-code-code.copied {
    opacity: 0;
}

.copy-code-box {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;

    background: var(--color-bg-weaker);
    border-radius: var(--border-radius);
    padding: 6px 10px;
}

.copy-code-box.copied {
    background: var(--color-success-bg);
}

.copy-code-button {
    display: flex;
    align-items: center;
    justify-content: center;

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

.copy-code-copied,
.copy-code-button.copied * {
    color: var(--color-success);
}
</style>
