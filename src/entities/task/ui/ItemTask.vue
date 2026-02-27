<script setup lang="ts">
import { ITask } from "../model/types";

interface Props {
  task: ITask;
}
const props = defineProps<Props>();
</script>
<template>
  <h3 text="gray-300" font="medium" w="full">
    {{ task.name }}
  </h3>
  <p text="gray-400" w="full">{{ task.description }}</p>
</template>
