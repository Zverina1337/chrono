<script setup lang="ts">
import {
  computed,
  ref,
  shallowRef,
  useTemplateRef,
  watch,
} from "vue";
import BaseInput from "./BaseInput.vue";
import { useClickOutside } from "../lib/useClickOutside";

interface Props {
  modelValue: string;
}

interface Emits {
  "update:modelValue": [value: string];
}

const props = defineProps<Props>();
const emits = defineEmits<Emits>();
// TODO: Debounce hook
// TODO: Search by tasks
const suggestions = ref<string[]>([
  "Добавить задачу1",
  "Добавить задачу2",
  "Добавить задачу3",
  "Добавить задачу4",
  "Добавить задачу5",
  "Добавить задачу6",
]);
const isOpen = shallowRef(false);
const activeIndex = shallowRef(0);

const incrementIndex = () => {
  if (activeIndex.value < suggestions.value.length - 1) {
    activeIndex.value++;
  }
};

const decrementIndex = () => {
  if (activeIndex.value > 0) {
    activeIndex.value--;
  }
};

const toggleList = () => {
  isOpen.value = !isOpen.value;
};

const changeValue = (index: number) => {
  value.value = suggestions.value[index];
  toggleList();
};

const value = computed({
  get: () => {
    return props.modelValue;
  },
  set: (value) => {
    emits("update:modelValue", value);
  },
});

const loading = shallowRef(true);

const root = useTemplateRef("root");
useClickOutside(root, () => (isOpen.value = false));

watch(isOpen, () => {
  const currentIndex = suggestions.value.findIndex(
    (suggestion) => suggestion === value.value,
  );
  activeIndex.value = currentIndex === -1 ? 0 : currentIndex;
});
</script>
<template>
  <div class="relative flex w-full items-center" ref="root">
    <BaseInput
      v-model="value"
      name="search-input"
      placeholder="Поиск..."
      input-class="bg-surface-sidebar shadow-sm rounded-lg"
      @click="toggleList()"
      @keydown.down.stop.prevent="incrementIndex"
      @keydown.up.stop.prevent="decrementIndex"
      @keydown.esc.stop="isOpen = false"
      @keydown.enter.stop="
        isOpen ? changeValue(activeIndex) : toggleList()
      "
    >
      <template #left-icon>
        <div class="i-mdi:magnify h-6 w-6 text-black/20" />
      </template>
      <template #right-icon>
        <div
          class="border-border bg-surface-sidebar flex items-center
            gap-1 rounded-xs border px-1 text-black/20"
        >
          <div class="i-mdi:apple-keyboard-command h-3 w-3" />
          <p class="font-medium">K</p>
        </div>
      </template>
    </BaseInput>
    <div
      v-if="isOpen"
      role="listbox"
      class="bg-surface-card absolute top-10 left-0 w-full rounded-md
        p-1 shadow-lg transition-opacity select-none"
    >
      <div v-if="suggestions.length !== 0">
        <p
          v-for="(suggestion, index) of suggestions"
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
          <span>{{ suggestion }}</span>
          <span
            v-if="suggestion === modelValue"
            class="i-mdi:check ml-auto block h-3 w-3"
          />
        </p>
      </div>
      <div v-else-if="value !== '' && !loading">
        Ничего не найдено!
      </div>
      <div v-else>Загрузка...</div>
    </div>
  </div>
</template>
