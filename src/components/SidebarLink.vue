<script setup lang="ts">
import { RouterLink } from 'vue-router';
defineProps({
  to: { type: String, required: true },
});
</script>

<template>
  <RouterLink class="truncate p-2 hover:bg-zinc-200/50 dark:hover:bg-zinc-700" :to="to"
    ><slot
  /></RouterLink>
</template>
