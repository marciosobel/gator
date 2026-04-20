<script setup lang="ts">
import WindowDecorations from "./Decorations.vue";
import TransferManager from "@/components/TransferManager.vue";
</script>

<template>
    <WindowDecorations />
    <main class="main-content">
        <TransferManager />
    </main>
</template>

<style scoped>
.main-content {
    display: flex;
    flex-direction: column;
    margin: auto;

    gap: 10px;
    width: min(90dvw, 400px);
}
</style>
