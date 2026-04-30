<script setup lang="ts">
import { IProject } from "../model/types";

const props = defineProps<{ project: IProject }>();
</script>
<!-- TODO: ACTIVE - shadow-xs bg-surface-card-->
<template>
  <article
    class="bg-surface-sidebar text-body cursor-pointer rounded-md px-3
      py-2 font-medium tracking-wide transition-colors
      hover:bg-black/10"
  >
    <h3 class="flex items-center gap-2">
      <span class="bg-brand-500 h-2 w-2 rounded-full" />
      <span class="capitalize">{{ project.name }}</span>
    </h3>
  </article>
</template>
