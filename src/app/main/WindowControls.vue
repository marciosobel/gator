<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Minus, Square, X } from "lucide-vue-next";
import DecorationButton from "./DecorationButton.vue";
import { closeMainWindow } from "@/events/main-window";

const window = getCurrentWebviewWindow();

const props = defineProps<{ iconSize: number }>();

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
</script>

<template>
    <DecorationButton @click="minimize">
        <Minus :size="props.iconSize" />
    </DecorationButton>
    <DecorationButton @click="toggleMaximize">
        <Square :size="props.iconSize" />
    </DecorationButton>
    <DecorationButton close @click="closeMainWindow">
        <X :size="props.iconSize" />
    </DecorationButton>
</template>
