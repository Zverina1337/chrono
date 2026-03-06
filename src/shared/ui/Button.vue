<script setup lang="ts"></script>
<template>
  <button
    class="hover:bg-indigo-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500"
    bg="indigo-500"
    transition="colors"
    text="white"
    p="2"
    rounded="lg"
    flex="~ items-center justify-around"
    outline="1 -~-offset-1 white/10 solid"
  >
    <slot></slot>
  </button>
</template>
