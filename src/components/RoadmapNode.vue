<script setup lang="ts">
defineProps({
  id: { type: Number, required: true },
  title: { type: String, required: true },
  description: { type: String, required: false },
});
</script>

<template>
  <div
    :data-node-id="id"
    class="rounded-md border-2 border-neutral-300 p-2 dark:border-neutral-700 dark:bg-neutral-800"
  >
    <p class="font-bold">{{ title }}</p>
    <p class="dark:text-neutral-400">{{ description }}</p>
  </div>
</template>
