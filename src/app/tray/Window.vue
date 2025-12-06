<script setup lang="ts">
import Dropzone, { type DropzonePayload } from "@/components/Dropzone.vue";
import PasteCode from "@/components/PasteCode.vue";
import Actions from "./Actions.vue";
import { ref } from "vue";

const files = ref<string[]>([]);

const onDrop = (payload: DropzonePayload) => {
    files.value = payload.paths;
};
</script>

<template>
    <main class="container">
        <Dropzone @files-dropped="onDrop" />
        <PasteCode />
        <div>
            <p style="font-size: 8px" v-for="file in files">{{ file }}</p>
        </div>
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
