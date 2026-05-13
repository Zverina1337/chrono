<script setup lang="ts" generic="T">
import {
  computed,
  shallowRef,
  useAttrs,
  useTemplateRef,
  watch,
} from "vue";
import type { SelectHTMLAttributes } from "vue";
import { useClickOutside } from "../lib/useClickOutside";

interface Option {
  label: string;
  disabled?: boolean;
  value: T;
}

interface Props {
  modelValue: Option;
  options: Array<Option>;
  selectClass?: string;
  labelClass?: string;
  rootClass?: string;
  errorClass?: string;
  name: string;
  error?: string;
  label?: string;
  size?: "sm" | "md" | "lg";
}

defineOptions({
  inheritAttrs: false,
});

const props = withDefaults(defineProps<Props>(), {
  size: "md",
});
const emits = defineEmits<{ "update:modelValue": [Option] }>();
const attrs: SelectHTMLAttributes = useAttrs();

const placeholder = computed(() => attrs.placeholder);
const rootClass = computed(() => attrs.class);
const rootStyle = computed(() => attrs.style);

const modelValue = computed({
  get: () => props.modelValue,
  set: (option: Option) => {
    emits("update:modelValue", option);
  },
});

const isOpen = shallowRef(false);

const toggleList = () => {
  isOpen.value = !isOpen.value;
};

const changeValue = (index: number) => {
  modelValue.value = props.options[index];
  toggleList();
};

const root = useTemplateRef("root");

const activeIndex = shallowRef(0);

const incrementIndex = () => {
  if (activeIndex.value < props.options.length - 1) {
    activeIndex.value++;
  }
};

const decrementIndex = () => {
  if (activeIndex.value > 0) {
    activeIndex.value--;
  }
};

useClickOutside(root, () => (isOpen.value = false));

const inputSizes = {
  sm: "input-sm",
  md: "input-md",
  lg: "input-lg",
};

watch(isOpen, () => {
  const currentIndex = props.options.findIndex(
    (option) => option.value === modelValue.value.value,
  );
  activeIndex.value = currentIndex === -1 ? 0 : currentIndex;
});
</script>

<template>
  <div
    :class="rootClass"
    :style="rootStyle"
    class="flex w-full flex-col gap-1"
    ref="root"
  >
    <label
      :class="labelClass"
      :for="name"
      v-if="label"
      class="text-input font-medium select-none"
    >
      {{ label }}
    </label>
    <!-- TODO: Собрать шорткат на этот инпут и сделать его в соответствии с дизайном -->
    <div
      tabindex="0"
      class="input-base relative flex cursor-pointer items-center
        justify-between"
      :class="[inputSizes[size], { 'shadow-focus': isOpen }]"
      @click="toggleList"
      @keydown.esc.stop="isOpen = false"
      @keydown.enter.stop="
        isOpen ? changeValue(activeIndex) : toggleList()
      "
      @keydown.down.stop.prevent="incrementIndex"
      @keydown.up.stop.prevent="decrementIndex"
    >
      <span v-if="modelValue.label">{{ modelValue.label }}</span>
      <p class="text-sm text-black/50 select-none" v-else>
        {{ placeholder }}
      </p>
      <div
        v-if="isOpen"
        role="listbox"
        class="bg-surface-card absolute top-10 left-0 w-full
          rounded-md p-1 shadow-lg transition-opacity select-none"
      >
        <p
          v-for="(option, index) of options"
          role="option"
          :class="[
            `text-input flex items-center rounded-md p-2 outline-none
            hover:bg-black/5`,
            {
              [`bg-brand-50 hover:bg-brand-50 text-brand-500
              font-medium`]: activeIndex === index,
            },
          ]"
          @click.stop="changeValue(index)"
        >
          <span>{{ option.label }}</span>
          <span
            v-if="option.value === modelValue"
            class="i-mdi:check ml-auto block h-3 w-3"
          />
        </p>
      </div>
      <div
        role="combobox"
        :class="[
          'i-mdi:chevron-up ml-auto h-5 w-4 transition-transform',
          { 'rotate-180': isOpen },
        ]"
      />
    </div>

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
