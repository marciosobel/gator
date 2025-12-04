<script setup>
import WindowDecorations from "./Decorations.vue";
import { invoke } from "@tauri-apps/api/core";

const close = async () => {
    await invoke("close_window");
};
</script>

<template>
    <WindowDecorations />
    <main>oi!</main>
</template>
