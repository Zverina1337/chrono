import { defineStore } from "pinia";
import { ref } from "vue";
import { IProject, IProjectActions } from "./types";
import { ProjectService } from "./service";

export const useProjectStore = defineStore("project", () => {
  const projects = ref<IProject[]>([]);

  const fetchProjects: IProjectActions["fetchProjects"] = async () => {
    const result = await ProjectService.getProjects();
    projects.value = [...result];
  };

  const createProject: IProjectActions["createProject"] = async (data) => {
    const result = await ProjectService.createProject(data);
    if (!result) return;
    projects.value = [...projects.value, result];
  };

  const updateProject: IProjectActions["updateProject"] = async (uuid, data) => {
    const result = await ProjectService.updateProject(uuid, data);
    if (!result) return;
    const foundedProject = projects.value.findIndex((project) => project.uuid === result.uuid);
    projects.value[foundedProject] = result;
  };

  const deleteProject: IProjectActions["deleteProject"] = async (uuid) => {
    const result = await ProjectService.deleteProject(uuid);
    if (result !== null) return;
    projects.value = projects.value.filter((project) => project.uuid === uuid);
  };

  return { projects, fetchProjects, createProject, updateProject, deleteProject };
});
