<script setup lang="ts">
import { computed, useAttrs } from "vue";
import type { TextareaHTMLAttributes } from "vue";

interface Props {
  modelValue: TextareaHTMLAttributes["value"];
  textareaClass?: string;
  labelClass?: string;
  rootClass?: string;
  errorClass?: string;
  descriptionClass?: string;
  name: string;
  error?: string;
  label?: string;
  description?: string;
}

defineOptions({
  inheritAttrs: false,
});

const props = defineProps<Props>();
const emits = defineEmits<{
  "update:modelValue": [value: TextareaHTMLAttributes["value"]];
}>();
const {
  class: rootClass,
  style: rootStyle,
  ...textareaAttrs
}: TextareaHTMLAttributes = useAttrs();

const modelValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:modelValue", value),
});
</script>

<template>
  <div :class="rootClass" :style="rootStyle">
    <label :class="labelClass" :for="name" v-if="label">{{
      label
    }}</label>
    <textarea
      v-model="modelValue"
      v-bind="textareaAttrs"
      type=""
      class="input-base h-20 resize-none px-3 py-2"
      :class="[
        textareaClass,
        {
          [`shadow-focus border-red-500 shadow-red-200
          focus-within:border-red-500 focus-within:outline-none`]:
            error,
        },
      ]"
      :name
      :id="name"
      :aria-describedby="error ? `${name}-error` : undefined"
      :aria-invalid="error ? true : undefined"
    />
    <p
      v-if="description"
      class="text-description text-xs"
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
