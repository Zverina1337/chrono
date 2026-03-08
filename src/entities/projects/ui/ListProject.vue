<script setup lang="ts">
import { useProjectStore } from "@/entities/projects/model/project";
import { storeToRefs } from "pinia";
import ItemProject from "./ItemProject.vue";
import FormProject from "./FormProject.vue";

const projeсtStore = useProjectStore();
const { projects } = storeToRefs(projeсtStore);
const { createProject } = projeсtStore;
</script>
<template>
  <section v-if="projects.length > 0" w="full">
    <ItemProject v-for="project in projects" :key="project.uuid" :project />
  </section>
  <section v-else>
    <FormProject @submit="(val) => createProject(val)" />
  </section>
</template>
