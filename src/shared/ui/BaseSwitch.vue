<script setup lang="ts">
import { computed, ref } from "vue";

interface Props {
  buttons: string[];
}

const props = defineProps<Props>();
const currentValue = ref(props.buttons[0]);
const activeIndex = computed(() =>
  props.buttons.indexOf(currentValue.value),
);
</script>
<template>
  <div
    class="border-border bg-surface-sidebar relative flex flex-1
      items-center rounded-md border p-1 text-sm font-medium
      text-black/40"
  >
    <div
      class="active-button border-border absolute inset-y-1 left-1
        rounded-sm border bg-white text-black"
      :style="{ '--index': activeIndex, '--count': buttons.length }"
    />

    <div
      v-for="button in buttons"
      :class="[
        `z-1 flex-1 cursor-pointer rounded-sm px-4 py-1
        transition-colors`,
        { 'text-black': currentValue === button },
      ]"
      @click="currentValue = button"
    >
      {{ button }}
    </div>
  </div>
</template>

<style scoped>
.active-button {
  width: calc((100% - 0.5rem) / var(--count));
  transform: translateX(calc(var(--index) * 100%));
  transition: transform 0.15s;
}
</style>
