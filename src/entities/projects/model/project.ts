import { defineStore } from "pinia";
import { ref } from "vue";
import { IProject } from "./types";

export const useProjectStore = defineStore("project", () => {
  const projects = ref<Array<IProject>>([]);

  const getProjects = () => {};

  return { projects };
});
