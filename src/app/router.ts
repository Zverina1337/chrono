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
  {
    path: "/",
    component: HomeView,
    name: "Tasks",
    icon: "formatList",
  },
  {
    path: "/projects",
    component: ProjectView,
    name: "Projects",
    icon: "formatList",
  },
];

export const router = createRouter({
  linkActiveClass: "active",
  linkExactActiveClass: "rail-item-active",
  history: createMemoryHistory(),
  routes,
});
