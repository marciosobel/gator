import { createRouter, createWebHistory } from "vue-router";
import TrayApp from "./app/TrayApp.vue";
import MainApp from "./app/MainApp.vue";

export const router = createRouter({
  routes: [
    { path: "/tray", component: TrayApp, alias: "/" },
    { path: "/main", component: MainApp },
  ],
  history: createWebHistory(),
});
