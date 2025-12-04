<template>
    <RouterView />
</template>

<style>
:root {
    --color-bg: #181820;
    --color-bg-weak: #1f1f28;
    --color-bg-active: #363646;
    --color-text: #dcd7ba;
    --color-border: rgba(255, 255, 255, 0.1);
    --color-danger: #c4746e;
    --color-danger-weak: #946160;

    --border-radius: 6px;
}

@font-face {
    font-family: "Nunito";
    src: url("./assets/fonts/Nunito.ttf");
    font-weight: normal;
    font-style: normal;
}

html,
body {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: var(--color-bg);
}

* {
    font-size: 1rem;
    color: var(--color-text);
    border: 0;
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
}

button {
    background: var(--color-bg-weak);
}

ul {
    padding: 0;
    margin: 0;
    list-style-type: none;
}
</style>
