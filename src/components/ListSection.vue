<script setup lang="ts">
import { ISection, ISectionActions, ITaskActions } from "../types/types";
import Button from "./Button.vue";
import ItemSection from "./ItemSection.vue";

interface Props {
  sections: ISection[];
}
interface Emits {
  addSection: Parameters<ISectionActions["addSection"]>;
  editSection: Parameters<ISectionActions["editSection"]>;
  addTask: Parameters<ITaskActions["addTask"]>;
  removeSection: Parameters<ISectionActions["removeSection"]>;
}

const props = defineProps<Props>();
const emits = defineEmits<Emits>();

const handleAddSection: ISectionActions["addSection"] = (name: ISection["name"]) => {
  emits("addSection", name);
};
const handleEditSection: ISectionActions["editSection"] = (uuid, data) => {
  emits("editSection", uuid, data);
};
const handleRemoveSection: ISectionActions["removeSection"] = (uuid) => {
  emits("removeSection", uuid);
};
const handleAddTask: ITaskActions["addTask"] = (sectionUuid, data) => {
  emits("addTask", sectionUuid, data);
};
</script>
<template>
  <section v-if="sections.length !== 0" class="w-full" flex="~ gap-2">
    <div
      v-for="section in sections"
      class="w-full h-full rounded bg-gray-700 p-5"
      flex="~ col gap-4"
    >
      <ItemSection
        :section
        @edit-section="handleEditSection"
        @remove-section="handleRemoveSection"
        @add-task="handleAddTask"
      />
    </div>
  </section>
  <section
    v-if="sections.length === 0"
    class="h-full w-full"
    flex="~ col items-center justify-center gap-4"
  >
    <h3 class="text-2xl text-white">У вас нет секций. Добавить?</h3>
    <Button class="px-4 py-2 font-medium" @click="handleAddSection">Добавить секцию</Button>
    <!-- <h3>У вас еще нет задач в этой секции. Добавить?</h3>
    <Button class="px-2 py-1 font-medium" @click="handleAddTask"> Добавить задачу </Button> -->
  </section>
</template>
