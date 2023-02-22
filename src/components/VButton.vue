<script setup lang="ts">
defineProps({
  accent: { type: Boolean },
});
</script>

<template>
  <button
    :class="{
      ' dark:border-emerald-500/20 dark:bg-emerald-500/40 dark:text-emerald-50 dark:hover:bg-emerald-500/60 dark:active:bg-emerald-500/20':
        accent,
      ' dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50 dark:hover:bg-neutral-700 dark:active:bg-neutral-900':
        !accent,
    }"
    class="rounded-md border px-2 py-1"
  >
    <slot />
  </button>
</template>
