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
    <!-- TODO: Собрать шорткат на этот инпут и сделать его в соответствии с дизайном -->

    <textarea
      v-model="modelValue"
      v-bind="textareaAttrs"
      type=""
      class="block w-full resize-none rounded-md bg-gray-700 px-3.5
        py-2 text-base text-white outline outline-1 outline-white/10
        placeholder:text-gray-500 focus:outline-2
        focus:-outline-offset-2 focus:outline-indigo-500"
      :class="textareaClass"
      :name
      :id="name"
      :aria-describedby="error ? `${name}-error` : undefined"
      :aria-invalid="error ? true : undefined"
    />
    <p
      v-if="error"
      :class="errorClass"
      :id="`${name}-error`"
      aria-live="assertive"
    >
      {{ error }}
    </p>
  </div>
</template>
