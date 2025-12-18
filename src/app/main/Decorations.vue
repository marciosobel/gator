<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Pin, PinOff } from "lucide-vue-next";
import DecorationButton from "./DecorationButton.vue";
import { onMounted, ref } from "vue";
import WindowControls from "./WindowControls.vue";

const ICON_SIZE = 16 as const;
const window = getCurrentWebviewWindow();
const pinnedToTop = ref(false);

const togglePinToTop = async () => {
    pinnedToTop.value = !pinnedToTop.value;
    await window.setAlwaysOnTop(pinnedToTop.value);
};

onMounted(async () => {
    pinnedToTop.value = await window.isAlwaysOnTop();
});
</script>

<template>
    <header class="window-decorations" data-tauri-drag-region>
        <span class="window-title">
            <img src="/icon.png" alt="Croc Icon" :width="20" />
            Gator
        </span>
        <span class="window-controls">
            <DecorationButton @click="togglePinToTop">
                <PinOff v-if="pinnedToTop" :size="ICON_SIZE" />
                <Pin v-else :size="ICON_SIZE" />
            </DecorationButton>

            <WindowControls :icon-size="ICON_SIZE" />
        </span>
    </header>
</template>

<style scoped>
.window-decorations {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
}

.window-controls {
    display: flex;
    gap: 5px;
}

.window-title {
    display: flex;
    align-items: center;
    gap: 8px;
    pointer-events: none;
}
</style>
