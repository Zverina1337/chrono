<script setup lang="ts">
import { onMounted } from "vue";
import { useProjectStore } from "@/entities/projects/model/project";
import ListProject from "@/entities/projects/ui/ListProject.vue";
const projectstore = useProjectStore();

onMounted(() => {
  projectstore.fetchProjects();
  console.log(projectstore.projects);
});
</script>
<template>
  <main h="screen" bg="gray-900" p="2" flex="~" gap="2">
    <ListProject />
  </main>
</template>
