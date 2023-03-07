<script setup lang="ts">
import { RouterLink } from 'vue-router';
defineProps({
  to: { type: String, required: true },
});
</script>

<template>
  <RouterLink
    class="truncate rounded-md p-2 ring-emerald-500/30 hover:ring-2 dark:ring-emerald-500/50"
    :to="to"
    ><slot
  /></RouterLink>
</template>
