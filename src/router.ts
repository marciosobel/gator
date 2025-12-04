import { createRouter, createWebHistory } from "vue-router";
import Tray from "@/app/tray/Window.vue";
import Main from "@/app/main/Window.vue";

export const router = createRouter({
  routes: [
    { path: "/tray", component: Tray, alias: "/" },
    { path: "/main", component: Main },
  ],
  history: createWebHistory(),
});
