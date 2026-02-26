<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useSectionStore } from "@/entities/section/model/section";
import { useTaskStore } from "@/entities/task/model/task";
import ListSection from "@/widgets/list-section/ListSection.vue";
import Button from "@/shared/ui/Button.vue";

const sectionStore = useSectionStore();
const taskStore = useTaskStore();
const { addTask } = taskStore;
const { sections } = storeToRefs(sectionStore);
const { addSection, removeSection, editSection } = sectionStore;
</script>
<template>
  <ListSection
    :sections
    @add-section="addSection('Новая секция')"
    @edit-section="editSection"
    @add-task="addTask"
    @remove-section="removeSection"
  />

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
