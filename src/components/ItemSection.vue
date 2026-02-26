<script setup lang="ts">
import { ISection, ISectionActions, ITaskActions } from "../types/types";
import { shallowRef, ShallowRef } from "vue";
import Button from "./Button.vue";

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
  editableSection.value = "";
  editableSectionValue.value = "";
};

const editableSection: ShallowRef<ISection["uuid"]> = shallowRef("");
const editableSectionValue: ShallowRef<string> = shallowRef("");

const handleSectionClick = (section: ISection) => {
  editableSection.value = section.uuid;
  editableSectionValue.value = section.name;
};
</script>

<template>
  <div flex="~ col gap-4">
    <div flex="~ justify-between items-center">
      <h2
        class="text-2xl text-gray-300 font-medium"
        v-show="editableSection !== section.uuid"
        @click="handleSectionClick(section)"
      >
        {{ section?.name }}
      </h2>
      <div
        class="flex justify-between items-center w-full gap-4"
        v-show="editableSection === section.uuid"
      >
        <input
          v-model="editableSectionValue"
          class="block w-full rounded-md bg-white/5 px-3.5 py-2 text-base text-white placeholder:text-gray-500 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500"
          text="white base"
          outline="1 -~-offset-1 white/10"
          type="text"
        />
        <Button @click="handleEditSection(section.uuid, { name: editableSectionValue })">
          Изменить
        </Button>
      </div>
      <Button
        v-show="editableSection !== section.uuid"
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
