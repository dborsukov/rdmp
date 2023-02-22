import { onMounted, onBeforeUnmount, type Ref } from 'vue';

export function useClickOutside(target_ref: Ref<HTMLElement>, callback: Function) {
  if (!target_ref) return;

  let listener = (e: MouseEvent) => {
    if (e.target == target_ref.value || e.composedPath().includes(target_ref.value)) {
      return;
    }
    callback();
  };

  onMounted(() => {
    window.addEventListener('click', listener);
  });
  onBeforeUnmount(() => {
    window.removeEventListener('click', listener);
  });
}

export {};
