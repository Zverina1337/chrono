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
  event.dataTransfer.setData("application/json", JSON.stringify(props.task));
};

const handleDrop = (event: DragEvent) => {
  if (!event.dataTransfer) return;
  const task = JSON.parse(event.dataTransfer.getData("application/json"));
  swapTask(task, props.task);
};
</script>
<template>
  <article
    w="full"
    border="~ white"
    bg="gray-600"
    p="5"
    rounded="md"
    flex="~ col"
    draggable="true"
    @dragstart="handleDragStart"
    @drop.stop="handleDrop"
  >
    <div flex="~" items="center" justify="between">
      <h3 text="gray-300" font="medium" w="full" select="none">
        {{ task.name }}
      </h3>
      <Button
        rounded="full"
        text="2xl"
        bg="white"
        class="i-material-symbols-light:cancel-outline"
        select="none"
        @click="deleteTask(task.sectionUuid, task.uuid)"
      />
    </div>
    <p text="gray-400" w="full" select="none">{{ task.description }}</p>
  </article>
</template>
