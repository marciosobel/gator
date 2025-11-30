<script setup>
import App from "./app/App.vue";
</script>

<template>
    <App />
</template>

<style>
:root {
    --color-bg: #181820;
    --color-bg-weak: #1f1f28;
    --color-text: #dcd7ba;
}

@font-face {
    font-family: "Nunito";
    src: url("./assets/fonts/Nunito.ttf");
    font-weight: normal;
    font-style: normal;
}

body {
    font-family:
        Nunito,
        system-ui,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        Roboto,
        Oxygen,
        Ubuntu,
        Cantarell,
        "Open Sans",
        "Helvetica Neue",
        sans-serif;
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: var(--color-bg);
    color: var(--color-text);
}

ul {
    padding: 0;
    margin: 0;
    list-style-type: none;
}
</style>
