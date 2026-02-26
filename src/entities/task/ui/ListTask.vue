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
  <article
    class="w-full border border-white rounded-lg bg-gray-600 p-5"
    v-for="task in getTasks(section.uuid)"
  >
    <ItemTask :task />
  </article>
</template>
