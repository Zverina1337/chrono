<script setup lang="ts">
interface Props {
  variant?:
    | "primary"
    | "secondary"
    | "ghost"
    | "icon"
    | "glass"
    | "disabled";
  size?: "sm" | "md" | "lg";
}
withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
});

const btnSizes = {
  sm: "btn-sm",
  md: "btn-md",
  lg: "btn-lg",
};
const btnVariants = {
  primary: "btn-primary",
  secondary: "btn-secondary",
  ghost: "btn-ghost",
  icon: "btn-icon",
  glass: "btn-glass",
  disabled: "btn-disabled",
};
</script>
<template>
  <button :class="[btnSizes[size], btnVariants[variant]]" class="btn">
    <slot></slot>
  </button>
</template>
