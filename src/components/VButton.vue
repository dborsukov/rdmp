<script setup lang="ts">
defineProps({
  accent: { type: Boolean },
});
</script>

<template>
  <button
    :class="{
      'border-emerald-500/80 bg-emerald-500 text-emerald-50 dark:border-emerald-500/20 dark:bg-emerald-500/40 dark:text-emerald-50 dark:hover:bg-emerald-500/60 dark:active:bg-emerald-500/20':
        accent,
      'border-gray-300 bg-gray-100 text-gray-700 hover:bg-gray-200 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50 dark:hover:bg-neutral-700 dark:active:bg-neutral-900':
        !accent,
    }"
    class="rounded-md border px-2 py-1"
  >
    <slot />
  </button>
</template>
