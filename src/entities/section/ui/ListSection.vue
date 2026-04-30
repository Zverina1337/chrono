<script setup lang="ts">
import ItemSection from "@/widgets/item-section/ItemSection.vue";
import { storeToRefs } from "pinia";
import { useSectionStore } from "../model/section";

const sectionStore = useSectionStore();

const { sections } = storeToRefs(sectionStore);
</script>
<template>
  <section v-if="sections.length !== 0" class="flex w-full gap-2">
    <ItemSection
      :key="section.uuid"
      :section
      v-for="section in sections"
    />
  </section>
  <section
    v-else
    class="flex h-full w-full flex-col items-center justify-center
      gap-4"
  >
    <h3 class="text-2xl text-white">У вас нет секций.</h3>
  </section>
</template>
