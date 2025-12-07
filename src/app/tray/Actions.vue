<script lang="ts" setup>
import { closeTray, openMainWindow } from "@/events";
import { LucideIcon, Maximize2, X } from "lucide-vue-next";
import { ref } from "vue";

type MenuButton = {
    icon?: LucideIcon;
    text: string;
    action: () => void | Promise<void>;
    accelerator?: string;
};

const buttons = ref<MenuButton[]>([
    {
        icon: Maximize2,
        text: "Open main window",
        action: () => openMainWindow({ hideTray: true }),
    },
    {
        icon: X,
        text: "Close",
        action: closeTray,
    },
]);
</script>

<template>
    <div class="menu-container">
        <button
            v-for="button in buttons"
            class="menu-button"
            @click="button.action"
            :key="button.text"
        >
            <component v-if="button.icon" :is="button.icon" :size="20" />
            <span>{{ button.text }}</span>
            <span v-if="button.accelerator">{{ button.accelerator }}</span>
        </button>
    </div>
</template>

<style scoped>
.menu-container {
    width: 90%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;

    margin-top: auto;
    margin-bottom: 10px;
    margin-inline: auto;
}

.menu-button {
    width: 100%;
    min-height: 24px;

    display: flex;
    align-items: center;
    gap: 8px;

    background: transparent;
    border-radius: var(--border-radius);
    padding: 8px 12px;
    cursor: pointer;
}

.menu-button:hover {
    background: var(--color-bg-weak);
}
</style>
