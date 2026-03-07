import { defineStore } from "pinia";
import { ref } from "vue";
import { ITask, ITaskActions } from "./types";

export const useTaskStore = defineStore("task", () => {
  const tasks = ref(new Map<string, ITask[]>());

  const getTasks: ITaskActions["getTasks"] = (projectUuid) => {
    const tasksBySectionId = tasks.value.get(projectUuid);
    if (tasksBySectionId) {
      return tasksBySectionId;
    } else {
      return [];
    }
  };

  const getTask: ITaskActions["getTask"] = (projectUuid, uuid) => {
    const tasksByProjectId = getTasks(projectUuid);
    if (!tasksByProjectId) return null;
    const task = tasksByProjectId.find((task) => task.uuid === uuid);
    if (task) {
      return task;
    } else {
      return null;
    }
  };

  function* getIncrement(): Generator<number> {
    let i = 0;
    while (true) {
      yield i++;
    }
  }
  const increment = getIncrement();

  const addTask: ITaskActions["addTask"] = (projectUuid, taskData) => {
    const createdAt = new Date().toDateString();
    const updatedAt = new Date().toDateString();

    const newTask = {
      uuid: crypto.randomUUID(),
      position: increment.next().value,
      createdAt,
      updatedAt,
      ...taskData,
    };
    const projectTasks = tasks.value.get(projectUuid);
    if (projectTasks) {
      projectTasks.push(newTask);
    } else {
      tasks.value.set(projectUuid, [newTask]);
    }
  };

  const deleteTask: ITaskActions["deleteTask"] = (projectUuid, uuid) => {
    const tasksByProjectId = getTasks(projectUuid);
    if (!tasksByProjectId.length) return;
    tasks.value.set(
      projectUuid,
      tasksByProjectId.filter((task) => task.uuid !== uuid),
    );
  };

  const moveTask: ITaskActions["moveTask"] = (task, toProjectUuid) => {
    deleteTask(task.projectUuid, task.uuid);
    task.projectUuid = toProjectUuid;
    tasks.value.set(toProjectUuid, [...getTasks(toProjectUuid), task]);
  };

  const swapTask: ITaskActions["swapTask"] = (task, targetTask) => {
    let projectTasks = [...getTasks(task.projectUuid)];
    let taskBuffer = targetTask;
    if (task.projectUuid !== targetTask.projectUuid) return;
    const taskIndex = projectTasks.findIndex((item) => item.uuid === task.uuid);
    const targetTaskIndex = projectTasks.findIndex((item) => item.uuid === targetTask.uuid);
    projectTasks[targetTaskIndex] = task;
    projectTasks[taskIndex] = taskBuffer;
    tasks.value.set(task.projectUuid, projectTasks);
  };

  return { getTasks, addTask, deleteTask, getTask, moveTask, swapTask };
});
