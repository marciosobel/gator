<script setup>
import { Clipboard } from "lucide-vue-next";
</script>

<template>
    <div class="insert-code-container">
        <input placeholder="Insert code" class="insert-code-input" />
        <button role="button" class="paste-button">
            <Clipboard :size="20" />
        </button>
    </div>
</template>

<style scoped>
.insert-code-container {
    display: flex;
    width: 90%;
    margin-inline: auto;

    height: 32px;
}

.insert-code-container > * {
    border: 1px solid var(--color-border);
}

.insert-code-input {
    width: 100%;
    border-radius: var(--border-radius) 0 0 var(--border-radius);
    padding-left: 10px;
    background: var(--color-bg-weak);
}

.paste-button {
    display: flex;
    align-items: center;
    justify-content: center;

    height: 100%;
    border-radius: 0 var(--border-radius) var(--border-radius) 0;
}
</style>
