import { createMemoryHistory, createRouter } from "vue-router";
import { IconKey } from "@/shared/types/icons";
import HomeView from "@/pages/HomeView.vue";
import { Component } from "vue";
import ProjectView from "@/pages/ProjectView.vue";

interface AppRoute {
  path: string;
  name: string;
  icon: IconKey;
  component: Component;
}

export const routes: AppRoute[] = [
  { path: "/", component: HomeView, name: "Tasks", icon: "formatList" },
  { path: "/projects", component: ProjectView, name: "Projects", icon: "formatList" },
];

export const router = createRouter({
  linkActiveClass: "active",
  linkExactActiveClass: "shadow-glow dark:bg-dark-emerald bg-white-emerald/50",
  history: createMemoryHistory(),
  routes,
});
