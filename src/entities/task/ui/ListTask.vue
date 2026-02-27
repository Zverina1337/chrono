<script setup lang="ts">
import { ISection } from "@/entities/section/model/types";
import { useTaskStore } from "../model/task";
import ItemTask from "./ItemTask.vue";

interface Props {
  section: ISection;
}
const props = defineProps<Props>();
const taskStore = useTaskStore();
const { getTasks } = taskStore;
</script>
<template>
  <article w="full" border="~ white" bg="gray-600" p="5" v-for="task in getTasks(section.uuid)">
    <ItemTask :task />
  </article>
</template>
