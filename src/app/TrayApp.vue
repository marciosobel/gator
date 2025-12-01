<script setup>
import Dropzone from "../components/Dropzone.vue";
import TrayButtons from "../components/TrayButtons.vue";
import PasteCode from "../components/PasteCode.vue";
</script>

<template>
    <main class="container">
        <Dropzone />
        <PasteCode />
        <TrayButtons />
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
