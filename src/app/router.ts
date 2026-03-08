import { createMemoryHistory, createRouter } from "vue-router";

import HomeView from "@/pages/HomeView.vue";
import ProjectView from "@/pages/ProjectView.vue";

const routes = [
  { path: "/", component: HomeView },
  { path: "/project/:uuid", component: ProjectView },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
