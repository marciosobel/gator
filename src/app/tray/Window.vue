<script setup>
import Dropzone from "@/components/Dropzone.vue";
import PasteCode from "@/components/PasteCode.vue";
import Actions from "./Actions.vue";
</script>

<template>
    <main class="container">
        <Dropzone />
        <PasteCode />
        <Actions />
    </main>
</template>

<style scoped>
.container {
    width: 100dvw;
    height: 100dvh;
    display: flex;
    flex-direction: column;
    gap: 10px;
}
</style>
