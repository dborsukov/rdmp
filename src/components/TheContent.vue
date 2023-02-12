<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<template>
  <div class="relative flex-1">
    <div
      class="absolute top-0 bottom-0 right-0 left-0 overflow-auto bg-white p-8 text-black scrollbar-thin scrollbar-track-transparent scrollbar-thumb-zinc-300 dark:bg-neutral-900 dark:text-neutral-50 dark:scrollbar-thumb-zinc-700"
    >
      <RouterView />
    </div>
  </div>
</template>
