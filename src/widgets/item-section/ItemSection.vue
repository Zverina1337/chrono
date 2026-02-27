<script setup lang="ts">
import { computed } from "vue";
import { ISection } from "@/entities/section/model/types";
import { useSectionStore } from "@/entities/section/model/section";
import { useTaskStore } from "@/entities/task/model/task";
import Button from "@/shared/ui/Button.vue";
import InlineEdit from "@/shared/ui/InlineEdit.vue";
import ListTask from "@/entities/task/ui/ListTask.vue";

interface Props {
  section: ISection;
}

const props = defineProps<Props>();

const sectionName = computed({
  get: () => props.section.name,
  set: (value) => editSection(props.section.uuid, { name: value }),
});

const sectionStore = useSectionStore();
const taskStore = useTaskStore();
const { removeSection, editSection } = sectionStore;
const { addTask } = taskStore;
</script>

<template>
  <div flex="~ col" gap="4">
    <div flex="~" items="center" justify="between">
      <InlineEdit :id="section.uuid" v-model="sectionName">
        <h2 text="2xl gray-300" font="medium">
          {{ section?.name }}
        </h2>
      </InlineEdit>
      <Button
        rounded="full"
        text="2xl"
        bg="white"
        class="i-material-symbols-light:cancel-outline"
        @click="removeSection(section.uuid)"
      />
    </div>
    <Button
      px="2"
      py="1"
      @click="
        addTask(section.uuid, {
          name: 'task-name',
          description: 'task-description',
        })
      "
    >
      Добавить задачу
    </Button>
  </div>
  <ListTask :section />
</template>
