<script setup lang="ts">
defineProps({
  id: { type: Number, required: true },
  title: { type: String, required: true },
});
</script>

<template>
  <div
    :data-node-id="id"
    class="rounded-md border-2 border-green-500 bg-green-700/50 p-2 text-center font-bold text-white"
  >
    <p>{{ title }}</p>
  </div>
</template>
