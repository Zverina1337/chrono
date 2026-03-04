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

  const getTask: ITaskActions["getTask"] = (sectionUuid, uuid) => {
    const tasksBySectionId = getTasks(sectionUuid);
    if (!tasksBySectionId) return null;
    const task = tasksBySectionId.find((task) => task.uuid === uuid);
    if (task) {
      return task;
    } else {
      return null;
    }
  };

  function* increment() {
    let i = 0;
    while (true) {
      i++;
      yield i;
    }
  }
  const addNameIncrement = increment();
  const addDescIncrement = increment();

  const addTask: ITaskActions["addTask"] = (sectionUuid, taskData) => {
    const newTask = {
      uuid: crypto.randomUUID(),
      sectionUuid,
      name: `${taskData["name"]} ${addNameIncrement.next().value}`,
      description: `${taskData["description"]} ${addDescIncrement.next().value}`,
    };
    const sectionTasks = tasks.value.get(sectionUuid);
    if (sectionTasks) {
      sectionTasks.push(newTask);
    } else {
      tasks.value.set(sectionUuid, [newTask]);
    }
  };

  const deleteTask: ITaskActions["deleteTask"] = (sectionUuid, uuid) => {
    const tasksBySectionId = getTasks(sectionUuid);
    if (!tasksBySectionId.length) return;
    tasks.value.set(
      sectionUuid,
      tasksBySectionId.filter((task) => task.uuid !== uuid),
    );
  };

  const moveTask: ITaskActions["moveTask"] = (task, toSectionUuid) => {
    deleteTask(task.sectionUuid, task.uuid);
    task.sectionUuid = toSectionUuid;
    tasks.value.set(toSectionUuid, [...getTasks(toSectionUuid), task]);
  };

  const swapTask: ITaskActions["swapTask"] = (task, targetTask) => {
    let sectionTasks = [...getTasks(task.sectionUuid)];
    let taskBuffer = targetTask;
    if (task.sectionUuid !== targetTask.sectionUuid) return;
    const taskIndex = sectionTasks.findIndex((item) => item.uuid === task.uuid);
    const targetTaskIndex = sectionTasks.findIndex((item) => item.uuid === targetTask.uuid);
    sectionTasks[targetTaskIndex] = task;
    sectionTasks[taskIndex] = taskBuffer;
    tasks.value.set(task.sectionUuid, sectionTasks);
  };

  return { getTasks, addTask, deleteTask, getTask, moveTask, swapTask };
});
