<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useSectionStore } from "@/entities/section/model/section";
import ListSection from "@/entities//section/ui/ListSection.vue";
import Button from "@/shared/ui/Button.vue";

const sectionStore = useSectionStore();
const { sections } = storeToRefs(sectionStore);
const { addSection } = sectionStore;
</script>
<template>
  <ListSection :sections />
  <!-- TODO: Когда начнем делать страницу проектов, важно чтобы использовались дизайн токены здвеь -->
  <Button
    class="p-2 font-medium tracking-wider [writing-mode:vertical-lr]"
    @click="addSection('Новая секция')"
  >
    Добавить секцию
  </Button>
</template>
