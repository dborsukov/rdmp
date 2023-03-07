<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<template>
  <div class="relative flex-1">
    <div
      class="absolute top-0 bottom-0 right-0 left-0 overflow-auto bg-gray-50 text-gray-800 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-gray-200 dark:bg-neutral-900 dark:text-neutral-50 dark:scrollbar-thumb-neutral-700"
    >
      <RouterView />
    </div>
  </div>
</template>
