<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Minus, Square, X } from "lucide-vue-next";
import DecorationButton from "./DecorationButton.vue";

const ICON_SIZE = 16 as const;
const window = getCurrentWebviewWindow();

const toggleMaximize = async () => {
    const isMaximizable = await window.isMaximizable();
    if (!isMaximizable) {
        return;
    }

    const isMaximized = await window.isMaximized();
    if (isMaximized) {
        await window.unmaximize();
    } else {
        await window.maximize();
    }
};

const minimize = async () => {
    const isMinimizable = await window.isMinimizable();
    if (isMinimizable) {
        await window.minimize();
    }
};

const close = async () => {
    await invoke("close_window");
};
</script>

<template>
    <header class="window-decorations" data-tauri-drag-region="true">
        <span class="window-title">Croc UI</span>
        <span class="window-controls">
            <DecorationButton @click="minimize">
                <Minus :size="ICON_SIZE" />
            </DecorationButton>
            <DecorationButton @click="toggleMaximize">
                <Square :size="ICON_SIZE" />
            </DecorationButton>
            <DecorationButton close @click="close">
                <X :size="ICON_SIZE" />
            </DecorationButton>
        </span>
    </header>
</template>

<style scoped>
.window-decorations {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
}

.window-controls {
    display: flex;
    gap: 5px;
}

.window-title {
    pointer-events: none;
}
</style>
