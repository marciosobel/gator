<script lang="ts" setup>
const props = defineProps<{
    progress?: number;
}>();
</script>

<template>
    <div class="progress-container">
        <div
            class="progress-bar"
            :style="{ width: (props.progress || 0) + '%' }"
        />
    </div>
</template>

<style scoped>
.progress-container {
    width: 100%;
    background-color: var(--color-bg-weaker);
    border-radius: var(--border-radius);
    overflow: hidden;
}

.progress-bar {
    height: 10px;
    background: var(--color-success);
    border-radius: var(--border-radius);
    transition: width 150ms ease-in-out;
}
</style>
