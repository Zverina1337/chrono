<script setup lang="ts">
import { IProject } from "../model/types";

const props = defineProps<{ project: IProject }>();
</script>

<template>
  <article bg="white-surface-card" shadow rounded-md p="1">
    <h3 text="primary lg">{{ project.name }}</h3>
  </article>
</template>
