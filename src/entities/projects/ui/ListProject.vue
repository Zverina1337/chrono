<script setup lang="ts">
import { useProjectStore } from "@/entities/projects/model/project";
import { storeToRefs } from "pinia";
import ItemProject from "./ItemProject.vue";
import { IProject } from "../model/types";
import { watch } from "vue";

const projeсtStore = useProjectStore();
const { projects } = storeToRefs(projeсtStore);

const emits = defineEmits<{ getId: [uuid: IProject["uuid"]] }>();

watch([projects], () => {
  if (projects.value.length !== 0) {
    emits("getId", projects.value[0].uuid);
  }
});
</script>
<template>
  <section
    v-if="projects.length !== 0"
    class="flex w-full flex-col gap-4 pt-1"
  >
    <h2 class="text-label flex items-center justify-between">
      <span>Проекты</span>
      <!-- // TODO: Create project -->
      <span class="i-mdi-plus block h-4 w-4 text-black/60" />
    </h2>
    <ItemProject
      v-for="project in projects"
      :key="project.uuid"
      :project
      @click="emits('getId', project.uuid)"
    />
  </section>
</template>
