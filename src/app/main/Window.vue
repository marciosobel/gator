<script setup>
import { invoke } from "@tauri-apps/api/core";

const close = async () => {
    await invoke("close_window");
};
</script>

<template>
    <main>oi!</main>
</template>
