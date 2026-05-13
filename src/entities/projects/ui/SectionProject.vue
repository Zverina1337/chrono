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
</script>
<template>
  <section
    v-if="project"
    class="bg-surface-card border-b-border flex h-20 w-full
      items-center justify-between border-b px-6"
  >
    <div class="flex w-1/4 flex-col gap-2">
      <h2 class="text-h1 font-bold capitalize">{{ project.name }}</h2>
      <p class="text-ink-tertiary text-sm">
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
  <section></section>
</template>
