<script setup lang="ts">
import Actions from "./Actions.vue";
import TransferManager from "@/components/TransferManager.vue";
</script>

<template>
    <main class="main-wrapper">
        <div class="main-content">
            <TransferManager compact />
        </div>
        <Actions />
    </main>
</template>

<style scoped>
.main-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    margin: 5dvw;
    height: 100dvh;
    gap: 10px;
}

.main-content {
    display: flex;
    flex-direction: column;
    gap: 10px;

    height: 100%;
}
</style>
