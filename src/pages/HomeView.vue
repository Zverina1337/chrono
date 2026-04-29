<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useProjectStore } from "@/entities/projects/model/project";
import ListProject from "@/entities/projects/ui/ListProject.vue";
const projectstore = useProjectStore();

onMounted(() => {
  projectstore.fetchProjects();
  console.log(projectstore.projects);
});

const filters = ref([
  { name: "Сегодня", count: 3 },
  { name: "На неделе", count: 5 },
  { name: "Все задачи", count: 8 },
]);

const badges = ref([
  { name: "backend" },
  { name: "frontend" },
  { name: "bug" },
  { name: "design" },
  { name: "docs" },
]);
</script>
<template>
  <main h="screen" flex="~" gap="2">
    <aside
      flex="~ col"
      gap="2"
      bg="white-surface-glass"
      p-5
      w="1/5"
      relative
      border-r="1px solid white-secondary/10"
    >
      <div flex="~ col" gap="4" border-b="1 solid white-secondary/10" pb="6">
        <h2 text="white-muted xs" tracking-wide uppercase font-medium>Фильтры</h2>
        <div flex="~ col" gap="3">
          <p
            v-for="filter in filters"
            text="white-tertiary sm"
            tracking-wide
            flex="~"
            justify="between"
            items="center"
          >
            <span>{{ filter.name }}</span>
            <span
              text="xs"
              w="5"
              h="4"
              bg="white-surface/10"
              flex="~"
              rounded="xs"
              justify="center"
              items="center"
              >{{ filter.count }}</span
            >
          </p>
        </div>
      </div>
      <ListProject />
      <div absolute bottom-2 flex="~ col" gap="4">
        <h2 text="white-muted xs" tracking-wide uppercase font-medium>Метки</h2>
        <div flex="~ col" gap="2" w="full">
          <p
            v-for="badge in badges"
            text="white-tertiary sm"
            tracking-wide
            flex="~"
            items="center"
            gap="2"
          >
            <span bg-white-emerald w="2" h="2" rounded-full />
            <span>{{ badge.name }}</span>
          </p>
        </div>
      </div>
    </aside>
    <section>
      <div>
        <div>
          <h1>Chrono</h1>
        </div>
      </div>
      <div></div>
    </section>
  </main>
</template>
