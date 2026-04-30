<script setup lang="ts">
import { ITask } from "../model/types";
import Button from "@/shared/ui/Button.vue";
import { useTaskStore } from "../model/task";

const props = defineProps<{ task: ITask }>();
const taskStore = useTaskStore();
const { deleteTask, swapTask } = taskStore;

const handleDragStart = (event: DragEvent) => {
  if (!event.dataTransfer) return;
  event.dataTransfer.effectAllowed = "move";
  event.dataTransfer.setData(
    "application/json",
    JSON.stringify(props.task),
  );
};

const handleDrop = (event: DragEvent) => {
  if (!event.dataTransfer) return;
  const task = JSON.parse(
    event.dataTransfer.getData("application/json"),
  );
  swapTask(task, props.task);
};
</script>
<template>
  <article
    class="flex w-full flex-col rounded-md border border-white
      bg-gray-600 p-5"
    draggable="true"
    @dragstart="handleDragStart"
    @drop.stop="handleDrop"
  >
    <div class="flex items-center justify-between">
      <!-- TODO: Подумай какой шорткат сюда подойдет -->
      <h3 class="gray-300 w-full font-medium select-none">
        {{ task.name }}
      </h3>
      <!-- TODO: Подумать над дизайном -->
      <Button
        class="i-mdi:window-close text-md rounded-full bg-white
          select-none"
        @click="deleteTask(task.sectionUuid, task.uuid)"
      />
    </div>
    <p class="w-full text-base select-none">
      {{ task.description }}
    </p>
  </article>
</template>
