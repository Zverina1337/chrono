<script setup lang="ts">
import { useTemplateRef, watchEffect } from "vue";
import Button from "@/shared/ui/Button.vue";

const props = defineProps<{ modelValue: boolean; title: string }>();
const emits = defineEmits<{
  "update:modelValue": [value: boolean];
}>();
const modal = useTemplateRef("dialog");

const onBackdropClick = (event: Event) => {
  if (event.target === modal.value) {
    modal.value?.close();
  }
};

const close = () => emits("update:modelValue", false);
watchEffect(() => {
  if (props.modelValue) modal.value?.showModal();
  else modal.value?.close();
});
</script>

<template>
  <!-- TODO: сделать его в соответствии с дизайном -->

  <dialog
    ref="dialog"
    @close="close"
    @click="onBackdropClick"
    class="rounded-xl bg-gray-800"
  >
    <div class="h-full w-full max-w-xl px-2.5 pb-2">
      <div class="flex items-center justify-between gap-4 pb-5">
        <!-- TODO: Подумать стоит ли тут добавлять шорткат -->
        <h2 class="text-2xl font-medium text-white">
          {{ title }}
        </h2>
        <!-- TODO: Подумать над дизайном -->
        <Button
          class="i-mdi:window-close text-md rounded-full bg-white"
          @click="close"
        />
      </div>
      <slot></slot>
    </div>
  </dialog>
</template>

<style scoped>
dialog {
  opacity: 0;
  transition:
    opacity 0.2s,
    display 0.2s allow-discrete,
    overlay 0.2s allow-discrete;
}

dialog[open] {
  opacity: 1;
}

@starting-style {
  dialog[open] {
    opacity: 0;
  }
}

dialog::backdrop {
  opacity: 0;
  backdrop-filter: blur(50px);
  transition:
    opacity 0.2s,
    display 0.2s allow-discrete,
    overlay 0.2s allow-discrete;
}

dialog[open]::backdrop {
  opacity: 0.5;
}

@starting-style {
  dialog[open]::backdrop {
    opacity: 0;
  }
}
</style>
