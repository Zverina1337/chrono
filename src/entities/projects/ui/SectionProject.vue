<script setup lang="ts">
import { shallowRef } from "vue";
import { IProject } from "../model/types";
import BaseSwitch from "@/shared/ui/BaseSwitch.vue";
import Button from "@/shared/ui/Button.vue";
import SearchInput from "@/shared/ui/SearchInput.vue";

interface Props {
  project: IProject;
}

const props = defineProps<Props>();
const search = shallowRef("");
const headers = ["Задача", "Метки", "Время"];

const badges = shallowRef([
  { name: "backend", color: "bg-tag-backend-fg" },
  { name: "frontend", color: "bg-tag-frontend-fg" },
  { name: "bug", color: "bg-tag-bug-fg" },
  { name: "design", color: "bg-tag-design-fg" },
  { name: "docs", color: "bg-tag-docs-fg" },
]);
</script>
<template>
  <div class="flex w-full flex-col" v-if="project">
    <section
      class="bg-surface-card border-b-border flex h-20 w-full
        items-center justify-between border-b px-6"
    >
      <div class="flex w-1/4 flex-col">
        <h2 class="text-header font-bold capitalize">
          {{ project.name }}
        </h2>
        <p class="text-description">
          <span>8 задач</span> ·
          <span>3 в работе</span>
        </p>
      </div>
      <div class="flex gap-3">
        <BaseSwitch :buttons="['Список', 'Канбан']" />
        <SearchInput v-model="search" name="search" />
        <Button>
          <div class="i-mdi-plus block h-4 w-4 text-white" />
          Задача
        </Button>
      </div>
    </section>
    <section class="p-6">
      <div
        class="bg-surface-card border-border w-full rounded-md border"
      >
        <table class="w-full">
          <thead class="border-border border-b">
            <tr>
              <th
                v-for="header of headers"
                class="text-align-start text-subheader w-1/3 p-3
                  first:w-[50%]"
              >
                {{ header }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td class="flex w-full items-center gap-2 p-3">
                <input type="checkbox" name="" id="" />
                <span class="h-2 w-2 rounded-full bg-red-500" />
                <span class="text-base">Дизайн главного экрана</span>
              </td>
              <td>
                <div>design</div>
                <div>frontend</div>
              </td>
              <td>2h 9m / 3h 0m</td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  </div>
</template>
