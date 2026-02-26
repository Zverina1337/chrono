import { onMounted, onUnmounted, ShallowRef } from "vue";

export const useClickOutside = (
  element: Readonly<ShallowRef<HTMLElement | null>>,
  callback: () => void,
) => {
  const checkElement = (event: Event) => {
    if (!element.value?.contains(event.target as Node)) {
      callback();
    }
  };
  onMounted(() => {
    document.addEventListener("click", checkElement);
  });
  onUnmounted(() => {
    document.removeEventListener("click", checkElement);
  });
};
