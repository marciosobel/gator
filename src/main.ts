import { createApp } from "vue";
import { router } from "./router";
import Root from "./app/Root.vue";

createApp(Root).use(router).mount("#root");
