<script setup lang="ts">
import { ISection, ISectionActions, ITaskActions } from "../types/types";
import { computed } from "vue";
import Button from "./Button.vue";
import InlineEdit from "./InlineEdit.vue";

interface Props {
  section: ISection;
}
interface Emits {
  editSection: Parameters<ISectionActions["editSection"]>;
  addTask: Parameters<ITaskActions["addTask"]>;
  removeSection: Parameters<ISectionActions["removeSection"]>;
}
const props = defineProps<Props>();
const emits = defineEmits<Emits>();

const handleAddTask: ITaskActions["addTask"] = (sectionUuid, data) => {
  emits("addTask", sectionUuid, { ...data });
};
const handleRemoveSection: ISectionActions["removeSection"] = (uuid) => {
  emits("removeSection", uuid);
};
const handleEditSection: ISectionActions["editSection"] = (uuid, data) => {
  emits("editSection", uuid, data);
};
const sectionName = computed({
  get: () => props.section.name,
  set: (value) => handleEditSection(props.section.uuid, { name: value }),
});
</script>

<template>
  <div flex="~ col gap-4">
    <div flex="~ justify-between items-center">
      <InlineEdit :id="section.uuid" v-model="sectionName">
        <h2 class="text-2xl text-gray-300 font-medium">
          {{ section?.name }}
        </h2>
      </InlineEdit>
      <Button
        class="rounded-full i-material-symbols-light:cancel-outline text-2xl bg-white"
        @click="handleRemoveSection(section.uuid)"
      />
    </div>
    <Button
      class="px-2 py-1"
      @click="
        handleAddTask(section.uuid, {
          name: 'task-name',
          description: 'task-description',
        })
      "
    >
      Добавить задачу
    </Button>
  </div>
</template>
