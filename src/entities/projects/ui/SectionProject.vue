<script setup lang="ts">
import { onMounted } from "vue";
import { IProject } from "../model/types";

interface Props {
  projectId: IProject["uuid"];
}

const props = defineProps<Props>();

onMounted(() => {});
</script>
<template>
  <div>
    {{ projectId }}
  </div>
  <!-- TODO: Когда начнем делать страницу проектов, важно чтобы использовались дизайн токены здвеь -->
</template>
