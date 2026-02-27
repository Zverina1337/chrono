<script setup lang="ts">
import ItemSection from "@/widgets/item-section/ItemSection.vue";
import { storeToRefs } from "pinia";
import { useSectionStore } from "../model/section";

const sectionStore = useSectionStore();

const { sections } = storeToRefs(sectionStore);
</script>
<template>
  <section v-if="sections.length !== 0" w="full" flex="~" gap="2">
    <div
      v-for="section in sections"
      w="full"
      h="full"
      rounded="~"
      bg="gray-700"
      p="5"
      flex="~ col"
      gap="4"
    >
      <ItemSection :key="section.uuid" :section />
    </div>
  </section>
  <section v-else h="full" w="full" flex="~ col" gap="4" items="center" justify="center">
    <h3 text="2xl white">У вас нет секций.</h3>
  </section>
</template>
