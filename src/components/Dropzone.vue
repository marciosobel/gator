<script setup>
import { Upload } from "lucide-vue-next";
</script>

<template>
    <label class="dropzone-area" for="dropzone">
        <Upload :size="24" />
        <span>Select a file to upload</span>
    </label>

    <input type="file" name="dropzone" style="display: none" />
</template>

<style scoped>
.dropzone-area {
    width: 80%;

    padding: 16px 12px;
    margin-top: 6%;
    margin-inline: auto;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    border: 2px dashed var(--color-text);
    border-radius: var(--border-radius);
    opacity: 70%;

    transition: opacity 100ms ease-in-out;
}

.dropzone-area:hover {
    opacity: 100%;
}
</style>
