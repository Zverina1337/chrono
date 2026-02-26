import { defineStore } from "pinia";
import { ref } from "vue";
import { ITask, ITaskActions } from "./types";

export const useTaskStore = defineStore("task", () => {
  const tasks = ref(new Map<string, ITask[]>());

  const getTasks: ITaskActions["getTasks"] = (sectionUuid) => {
    const tasksBySectionId = tasks.value.get(sectionUuid);
    if (tasksBySectionId) {
      return tasksBySectionId;
    } else {
      return [];
    }
  };

  const addTask: ITaskActions["addTask"] = (sectionUuid, taskData) => {
    const newTask = {
      uuid: crypto.randomUUID(),
      sectionUuid,
      ...taskData,
    };
    const sectionTasks = tasks.value.get(sectionUuid);
    if (sectionTasks) {
      sectionTasks.push(newTask);
    } else {
      tasks.value.set(sectionUuid, [newTask]);
    }
  };

  return { getTasks, addTask };
});
