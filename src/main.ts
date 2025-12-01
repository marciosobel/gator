import { createApp } from "vue";
import { router } from "./router";
import Root from "./Root.vue";

createApp(Root).use(router).mount("#root");
