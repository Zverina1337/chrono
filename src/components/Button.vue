<script setup>
import { useAttrs } from "vue";
const attrs = useAttrs();
</script>
<template>
  <button
    class="bg-indigo-500 hover:bg-indigo-400 transition-colors text-white p-2 rounded-lg"
    flex="~ items-center justify-around"
    v-bind:attrs="attrs"
  >
    <slot></slot>
  </button>
</template>
