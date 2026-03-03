<script setup lang="ts">
import { ITask } from "../model/types";
import Button from "@/shared/ui/Button.vue";
import { useTaskStore } from "../model/task";

interface Props {
  task: ITask;
}
const props = defineProps<Props>();
const taskStore = useTaskStore();
const { deleteTask } = taskStore;
</script>
<template>
  <article w="full" border="~ white" bg="gray-600" p="5" rounded="md" flex="~ col">
    <div flex="~" items="center" justify="between">
      <h3 text="gray-300" font="medium" w="full">
        {{ task.name }}
      </h3>
      <Button
        rounded="full"
        text="2xl"
        bg="white"
        class="i-material-symbols-light:cancel-outline"
        @click="deleteTask(task.sectionUuid, task.uuid)"
      />
    </div>
    <p text="gray-400" w="full">{{ task.description }}</p>
  </article>
</template>
