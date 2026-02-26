<script setup lang="ts">
import { computed, shallowRef, ref } from "vue";
import { useClickOutside } from "../composables/useClickOutside";

interface Props {
  id: string;
  modelValue: string;
}

interface Emits {
  "update:model-value": [value: string];
}

const props = defineProps<Props>();
const emits = defineEmits<Emits>();

const editable = shallowRef("");
const editableValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:model-value", value),
});

const handleClick = () => {
  editable.value = props.id;
};
const container = ref();

useClickOutside(container, () => (editable.value = ""));
</script>
<template>
  <div ref="container">
    <span v-show="editable !== id" @click="handleClick">
      <slot></slot>
    </span>
    <span v-show="editable === id" flex="~ justify-between items-center gap-4">
      <input
        ref="input"
        v-model="editableValue"
        class="block w-full rounded-md bg-white/5 px-3.5 py-2 text-base text-white placeholder:text-gray-500 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500"
        text="white base"
        outline="1 -~-offset-1 white/10"
        type="text"
      />
    </span>
  </div>
</template>
