<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useProjectStore } from "@/entities/projects/model/project";
import ListProject from "@/entities/projects/ui/ListProject.vue";
import SectionProject from "@/entities/projects/ui/SectionProject.vue";

const projectstore = useProjectStore();

const filters = ref([
  { name: "Сегодня", count: 3 },
  { name: "На неделе", count: 5 },
  { name: "Все задачи", count: 8 },
]);

const badges = ref([
  { name: "backend", color: "bg-tag-backend-fg" },
  { name: "frontend", color: "bg-tag-frontend-fg" },
  { name: "bug", color: "bg-tag-bug-fg" },
  { name: "design", color: "bg-tag-design-fg" },
  { name: "docs", color: "bg-tag-docs-fg" },
]);

const currentProject = ref();

onMounted(() => {
  projectstore.fetchProjects();
});
</script>
<template>
  <main class="flex h-screen">
    <aside
      class="bg-surface-sidebar border-r-border relative flex w-1/4
        flex-col gap-2 border-r p-5"
    >
      <div class="border-b-border flex flex-col gap-4 border-b pb-6">
        <h2 class="text-label">Фильтры</h2>
        <div class="flex flex-col gap-3">
          <p
            v-for="filter in filters"
            class="text-body-muted flex items-center justify-between
              font-medium"
          >
            <span>{{ filter.name }}</span>
            <span
              class="flex h-4 w-5 items-center justify-center
                rounded-xs bg-black/10 text-xs"
              >{{ filter.count }}</span
            >
          </p>
        </div>
      </div>
      <ListProject @get-id="(uuid) => (currentProject = uuid)" />
      <div class="absolute bottom-4 flex flex-col gap-4">
        <h2 class="text-label">Метки</h2>
        <div class="flex w-full flex-col gap-2">
          <p
            v-for="badge in badges"
            class="text-body flex items-center gap-2"
          >
            <span :class="badge.color" class="h-2 w-2 rounded-full" />
            <span>{{ badge.name }}</span>
          </p>
        </div>
      </div>
    </aside>
    <SectionProject :project="currentProject" />
  </main>
</template>
