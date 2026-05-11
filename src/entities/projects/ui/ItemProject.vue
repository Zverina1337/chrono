<script setup lang="ts">
import { IProject } from "../model/types";

const props = defineProps<{ project: IProject }>();
</script>
<!-- TODO: ACTIVE - nav-item-active -->
<template>
  <article class="nav-item">
    <h3 class="flex items-center gap-2">
      <span class="bg-brand-500 h-2 w-2 rounded-full" />
      <span class="capitalize">{{ project.name }}</span>
    </h3>
  </article>
</template>
