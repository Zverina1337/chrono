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

  <Button
    class="[writing-mode:vertical-lr]"
    p="2"
    font="medium"
    tracking="wider"
    @click="addSection('Новая секция')"
  >
    Добавить секцию
  </Button>
</template>
