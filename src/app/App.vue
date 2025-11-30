<script setup>
import Dropzone from "../components/Dropzone.vue";
import MenuButtons from "../components/MenuButtons.vue";
import PasteCode from "../components/PasteCode.vue";
</script>

<template>
    <main class="container">
        <Dropzone />
        <PasteCode />
        <MenuButtons />
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
