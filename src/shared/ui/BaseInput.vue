<script setup lang="ts" generic="T">
import { computed, useAttrs } from "vue";
import type { InputHTMLAttributes } from "vue";

interface Props {
  modelValue: T;
  inputClass?: string;
  labelClass?: string;
  rootClass?: string;
  errorClass?: string;
  name: string;
  type?: Exclude<
    InputHTMLAttributes["type"],
    "button" | "checkbox" | "radio" | "range" | "reset" | "search" | "submit"
  >;
  error?: string;
  label?: string;
}

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<Props>();
const emits = defineEmits<{ "update:modelValue": [value: T] }>();
const { class: rootClass, style: rootStyle, ...inputAttrs }: InputHTMLAttributes = useAttrs();

const modelValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:modelValue", value),
});
</script>

<template>
  <div :class="rootClass" :style="rootStyle">
    <label :class="labelClass" :for="name" v-if="label">{{ label }}</label>
    <input
      v-model="modelValue"
      v-bind="inputAttrs"
      :type="type ?? 'text'"
      class="placeholder:text-gray-500 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500"
      px="3.5"
      py="2"
      bg="gray-700"
      rounded="md"
      display="block"
      w="full"
      text="white base"
      outline="1 -~-offset-1 white/10 solid"
      :class="inputClass"
      :name
      :id="name"
      :aria-describedby="error ? `${name}-error` : undefined"
      :aria-invalid="error ? true : undefined"
    />
    <p v-if="error" :class="errorClass" :id="`${name}-error`" aria-live="assertive">
      {{ error }}
    </p>
  </div>
</template>
