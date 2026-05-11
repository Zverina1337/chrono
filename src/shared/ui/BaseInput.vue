<script setup lang="ts" generic="T">
import { computed, useAttrs } from "vue";
import type { InputHTMLAttributes } from "vue";

interface Props {
  modelValue: T;
  inputClass?: string;
  labelClass?: string;
  rootClass?: string;
  errorClass?: string;
  descriptionClass?: string;
  name: string;
  type?: Exclude<
    InputHTMLAttributes["type"],
    | "button"
    | "checkbox"
    | "radio"
    | "range"
    | "reset"
    | "search"
    | "submit"
  >;
  error?: string;
  label?: string;
  size?: "sm" | "md" | "lg";
  description?: string;
}

defineOptions({
  inheritAttrs: false,
});

const props = withDefaults(defineProps<Props>(), {
  size: "md",
});
const emits = defineEmits<{ "update:modelValue": [value: T] }>();
const {
  class: rootClass,
  style: rootStyle,
  ...inputAttrs
}: InputHTMLAttributes = useAttrs();

const modelValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:modelValue", value),
});

const inputSizes = {
  sm: "input-sm",
  md: "input-md",
  lg: "input-lg",
};
</script>

<template>
  <div
    class="flex w-full flex-col gap-1"
    :class="rootClass"
    :style="rootStyle"
  >
    <label
      class="text-input font-medium"
      :class="labelClass"
      :for="name"
      v-if="label"
    >
      {{ label }}
    </label>
    <div
      class="input-base flex items-center justify-around gap-1.5"
      :class="[
        inputClass,
        inputSizes[size],
        {
          [`shadow-focus border-red-500 shadow-red-200
          focus-within:border-red-500 focus-within:outline-none`]:
            error,
        },
      ]"
    >
      <slot name="left-icon"></slot>
      <input
        v-model="modelValue"
        v-bind="inputAttrs"
        :type="type ?? 'text'"
        class="h-full w-full outline-none"
        :class="inputClass"
        :name
        :id="name"
        :aria-describedby="error ? `${name}-error` : undefined"
        :aria-invalid="error ? true : undefined"
      />
      <slot name="right-icon"></slot>
    </div>
    <p
      v-if="description"
      class="text-caption text-xs"
      :class="descriptionClass"
    >
      {{ description }}
    </p>
    <p
      v-if="error"
      class="text-xs font-medium text-red-600"
      :class="errorClass"
      :id="`${name}-error`"
      aria-live="assertive"
    >
      {{ error }}
    </p>
  </div>
</template>
