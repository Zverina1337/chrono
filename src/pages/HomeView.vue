<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useSectionStore } from "@/entities/section/model/section";
import { useTaskStore } from "@/entities/task/model/task";
import ListSection from "@/widgets/list-section/ListSection.vue";

const sectionStore = useSectionStore();
const taskStore = useTaskStore();
const { addTask } = taskStore;
const { sections } = storeToRefs(sectionStore);
const { addSection, removeSection, editSection } = sectionStore;
</script>
<template>
  <ListSection
    :sections
    @add-section="addSection('some')"
    @edit-section="editSection"
    @add-task="addTask"
    @remove-section="removeSection"
  />
  <!-- 

    <article
      class="w-full border border-white rounded-lg bg-gray-600 p-5"
      v-for="task in getTasks(section.uuid)"
    >
      <h3 class="text-gray-300 font-medium w-full">{{ task.name }}</h3>
      <p class="text-gray-300 w-full">{{ task.description }}</p>
    </article> -->

  <button
    class="bg-indigo-500 hover:bg-indigo-400 transition-colors text-md [writing-mode:vertical-lr] rounded-lg text-white font-medium tracking-wider p-2"
    @click="addSection('some')"
  >
    Добавить секцию
  </button>
</template>
