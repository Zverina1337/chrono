<script setup lang="ts">
import { ISection } from "@/entities/section/model/types";
import { useTaskStore } from "../model/task";
import ItemTask from "./ItemTask.vue";

const props = defineProps<{ section: ISection }>();
const taskStore = useTaskStore();
const { getTasks } = taskStore;
</script>
<template>
  <ItemTask :task v-for="task in getTasks(section.uuid)" :key="task.uuid" />
</template>
