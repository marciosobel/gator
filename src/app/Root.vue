<template>
    <RouterView />
</template>

<style>
#root {
    display: flex;
    flex-direction: column;
    height: 100dvh;
}

:root {
    --color-bg: #272e33;
    --color-bg-weaker: #2e383c;
    --color-bg-weak: #374145;
    --color-bg-active: #495156;
    --color-text: #d3c6aa;
    --color-border: rgba(255, 255, 255, 0.1);
    --color-danger: #e67e80;
    --color-danger-bg: #4c3743;
    --color-danger-bg-active: #6b324e;
    --color-success: #a7c080;
    --color-success-bg: #3c4841;

    --border-radius: 6px;
}

@font-face {
    font-family: "Nunito";
    src: url("../assets/fonts/Nunito.ttf");
    font-weight: normal;
    font-style: normal;
    font-display: block;
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
    background: var(--color-bg-weaker);
    transition: background 100ms ease;
    padding: 8px 10px;
}

button:hover {
    background: var(--color-bg-weak);
}

button:active {
    background: var(--color-bg-active);
}

ul {
    padding: 0;
    margin: 0;
    list-style-type: none;
}
</style>
