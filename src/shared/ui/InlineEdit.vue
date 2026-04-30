<script setup lang="ts">
import { computed, shallowRef, useTemplateRef, nextTick } from "vue";
import { useClickOutside } from "@/shared/lib/useClickOutside";

interface Props {
  id: string;
  modelValue: string;
}

interface Emits {
  "update:model-value": [value: string];
}

const props = defineProps<Props>();
const emits = defineEmits<Emits>();

const editableId = shallowRef("");
const editableValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:model-value", value),
});

const input = useTemplateRef<HTMLInputElement>("input");

const handleClick = async () => {
  editableId.value = props.id;
  await nextTick();
  input.value?.focus();
};

const container = useTemplateRef<HTMLElement>("container");

useClickOutside(container, () => (editableId.value = ""));

const name = crypto.randomUUID();
</script>
<template>
  <!-- TODO: сделать его в соответствии с дизайном -->

  <div ref="container">
    <span v-show="editableId !== id" @click="handleClick">
      <slot></slot>
    </span>
    <span
      v-show="editableId === id"
      class="flex items-center justify-between gap-4"
    >
      <input
        v-model="editableValue"
        ref="input"
        class="block w-full rounded-md bg-gray-700 px-3.5 py-2
          text-base text-white outline outline-1 outline-white/10
          placeholder:text-gray-500 focus:outline-2
          focus:-outline-offset-2 focus:outline-indigo-500"
        type="text"
        :name
        @keydown.enter="editableId = ''"
        @keydown.escape="editableId = ''"
      />
    </span>
  </div>
</template>
