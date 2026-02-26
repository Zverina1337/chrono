import { onMounted, onUnmounted, Ref } from "vue";

export const useClickOutside = (element: Ref<HTMLElement | null>, callback: () => void) => {
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
