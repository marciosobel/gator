<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import Actions from "./Actions.vue";
import PasteCode from "@/components/PasteCode.vue";
import { receiveFiles, sendFiles } from "@/events";

const onDrop = ({ paths }: DropzonePayload) => {
    sendFiles(paths);
};

const onReceive = async (code: string) => {
    await receiveFiles(code);
};
</script>

<template>
    <main class="container">
        <Dropzone @files-dropped="onDrop" />
        <PasteCode @receive="onReceive" />
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
