<script setup lang="ts" generic="T">
import { computed, useAttrs } from "vue";
import type { OptionHTMLAttributes, SelectHTMLAttributes } from "vue";

interface Option extends Omit<OptionHTMLAttributes, "value"> {
  value: T;
}

interface Props {
  modelValue: T;
  options: Array<Option>;
  selectClass?: string;
  labelClass?: string;
  rootClass?: string;
  errorClass?: string;
  name: string;
  error?: string;
  label?: string;
}

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<Props>();
const emits = defineEmits<{ "update:modelValue": [value: T] }>();
const { class: rootClass, style: rootStyle, ...selectAttrs }: SelectHTMLAttributes = useAttrs();

const modelValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:modelValue", value),
});
</script>

<template>
  <div :class="rootClass" :style="rootStyle">
    <label :class="labelClass" :for="name" v-if="label">{{ label }}</label>
    <select
      v-model="modelValue"
      v-bind="selectAttrs"
      :class="['select', selectClass]"
      px="3.5"
      py="2"
      bg="gray-700"
      rounded="md"
      display="block"
      w="full"
      text="white base"
      :name
      :id="name"
      :aria-describedby="error ? `${name}-error` : undefined"
      :aria-invalid="error ? true : undefined"
    >
      <option
        v-for="(option, index) in options"
        v-bind="options[index]"
        :key="`${option.value}-${index}`"
      />
    </select>
    <p v-if="error" :class="errorClass" :id="`${name}-error`" aria-live="assertive">
      {{ error }}
    </p>
  </div>
</template>

<style scoped>
.select {
  appearance: base-select;
}

.select::placeholder {
  color: #6b7280;
}

.select:focus-visible {
  outline: 2px solid #6366f1;
}

.select:open {
  outline: 2px solid #6366f1;
}

.select::picker-icon {
  transition: transform 0.2s;
  color: white;
}
.select:open::picker-icon {
  transform: rotate(180deg);
}
.select > option {
  padding: 0.5rem;
}
.select > option:hover {
  background-color: #6366f1;
}
.select > option:checked {
  background-color: #6366f1;
}
.select > option::checkmark {
  display: none;
}
</style>
<style>
.select::picker(select) {
  appearance: base-select;
  background-color: #374151;
  border-radius: 0.375rem;
  border: 2px solid #6366f1;
  margin: 0.5rem 0;
  color: white;
  position-try-fallbacks: none;
}

.select::picker(select) {
  clip-path: inset(0 0 100% 0);
  transition:
    clip-path 0.25s ease,
    display 0.25s allow-discrete,
    overlay 0.25s allow-discrete;
}

.select:open::picker(select) {
  clip-path: inset(0 0 0% 0 round 8px);
}

@starting-style {
  .select:open::picker(select) {
    clip-path: inset(0 0 100% 0 round 8px);
  }
}
</style>
