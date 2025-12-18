<script setup lang="ts">
import { ref } from "vue";

const hover = ref(false);
const emit = defineEmits<{ click: [] }>();
const props = defineProps<{
    close?: boolean;
}>();

const handleClick = () => {
    hover.value = false;
    emit("click");
};
</script>

<template>
    <button
        @mouseover="hover = true"
        @mouseleave="hover = false"
        @click="handleClick"
        :class="{ hover, close: props.close }"
        class="window-button"
    >
        <slot />
    </button>
</template>

<style scoped>
.window-button {
    display: flex;
    align-items: center;
    justify-content: center;
    aspect-ratio: 1 / 1;
    border-radius: var(--border-radius);
    background: transparent;
    transition: background 75ms ease;
    cursor: default;
}

.window-button.close.hover {
    background: var(--color-danger-bg);
}

.window-button.close:active {
    background: var(--color-danger-bg-active);
    color: var(--color-danger);
}

.window-button:hover:not(.close) {
    background: var(--color-bg-weak);
}

.window-button:active:not(.close) {
    background: var(--color-bg-active);
}
</style>
