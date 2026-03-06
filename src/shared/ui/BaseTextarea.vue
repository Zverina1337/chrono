<script setup lang="ts">
import { computed, useAttrs } from "vue";
import type { TextareaHTMLAttributes } from "vue";

interface Props {
  modelValue: TextareaHTMLAttributes["value"];
  textareaClass?: string;
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
const emits = defineEmits<{ "update:modelValue": [value: TextareaHTMLAttributes["value"]] }>();
const { class: rootClass, style: rootStyle, ...textareaAttrs }: TextareaHTMLAttributes = useAttrs();

const modelValue = computed({
  get: () => props.modelValue,
  set: (value) => emits("update:modelValue", value),
});
</script>

<template>
  <div :class="rootClass" :style="rootStyle">
    <label :class="labelClass" :for="name" v-if="label">{{ label }}</label>
    <textarea
      v-model="modelValue"
      v-bind="textareaAttrs"
      type=""
      class="placeholder:text-gray-500 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500"
      px="3.5"
      py="2"
      bg="gray-700"
      rounded="md"
      display="block"
      w="full"
      text="white base"
      outline="1 -~-offset-1 white/10 solid"
      resize="none"
      :class="textareaClass"
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
