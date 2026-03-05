<script setup lang="ts">
import { computed, shallowRef } from "vue";
import { ISection } from "@/entities/section/model/types";
import { useSectionStore } from "@/entities/section/model/section";
import { useTaskStore } from "@/entities/task/model/task";
import Button from "@/shared/ui/Button.vue";
import InlineEdit from "@/shared/ui/InlineEdit.vue";
import ListTask from "@/entities/task/ui/ListTask.vue";
import { ITask } from "@/entities/task/model/types";
import Modal from "@/shared/ui/Modal.vue";
import FormTask from "@/entities/task/ui/FormTask.vue";

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
const { addTask, moveTask } = taskStore;
const isDragging = shallowRef(false);

const handleDrop = (event: DragEvent) => {
  isDragging.value = false;
  const target = event.target as HTMLElement | null;
  if (!event.dataTransfer || !target) return;
  const task = JSON.parse(event.dataTransfer.getData("application/json")) as ITask;
  moveTask(task, props.section.uuid);
};

const handleDragLeave = (event: DragEvent) => {
  const target = event.target as HTMLElement | null;
  const relatedTarget = event.relatedTarget as HTMLElement | null;
  if (!target || !relatedTarget) return;

  if (!target.contains(relatedTarget)) {
    isDragging.value = false;
  }
};
const addTaskModal = shallowRef(false);
</script>

<template>
  <div w="full" h="full" rounded="~" bg="gray-700" p="5" flex="~ col" gap="4">
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
      <Button px="2" py="1" @click="addTaskModal = true"> Добавить задачу </Button>
    </div>
    <div
      :class="
        isDragging
          ? 'border-green-500 border-dashed border-2 bg-green-500/10 rounded-lg'
          : 'border-green-500 border-dashed border-none bg-transparent'
      "
      flex="~ col"
      gap="4"
      h="full"
      w="full"
      p="1"
      transition="colors"
      @dragenter="() => (isDragging = true)"
      @dragend="() => (isDragging = false)"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
      @dragover.prevent
    >
      <ListTask :section />
    </div>
  </div>
  <Modal v-model="addTaskModal">
    <FormTask />
  </Modal>
</template>
