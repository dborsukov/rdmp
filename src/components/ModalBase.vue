<script setup lang="ts">
import { ref } from 'vue';

const isVisible = ref(false);

defineExpose({ open, close });

function open() {
  isVisible.value = true;
}

function close() {
  isVisible.value = false;
}
</script>

<template>
  <div
    v-if="isVisible"
    class="fixed top-0 bottom-0 right-0 left-0 z-50 flex flex-col items-center justify-center bg-black/40"
  >
    <div
      class="rounded-lg border bg-white p-4 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
    >
      <slot></slot>
    </div>
  </div>
</template>
