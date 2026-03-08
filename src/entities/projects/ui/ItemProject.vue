<script setup lang="ts">
import { IProject } from "../model/types";

const props = defineProps<{ project: IProject }>();
</script>

<template>
  <article w="full" max-h="8rem" h="full" rounded="~" bg="gray-700" p="5" flex="~ col" gap="4">
    <h2 text="white 2xl">{{ project.name }}</h2>
    <p>{{ project.description }}</p>
  </article>
</template>
