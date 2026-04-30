<script setup lang="ts">
interface Props {
  variant?: "primary" | "secondary" | "ghost" | "icon";
  size?: "sm" | "md" | "lg";
}
withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
});
</script>
<template>
  <button :class="`btn-${variant} btn-${size}`" class="btn">
    <slot></slot>
  </button>
</template>
<!-- TODO: Dynamic string class problem UnoCss base( -->
