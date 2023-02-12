<script setup lang="ts">
import { useUiStore } from '@/stores/ui';
import Link from '@/components/SidebarLink.vue';

const ui_store = useUiStore();
</script>

<template>
  <div
    v-show="ui_store.sidebarExpanded"
    class="flex w-64 flex-shrink-0 flex-col bg-white text-black dark:bg-neutral-800 dark:text-neutral-50"
  >
    <div class="relative flex-1">
      <div
        class="absolute top-0 right-0 left-0 bottom-0 flex flex-col gap-y-2 overflow-auto border-r border-neutral-200 p-4 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-neutral-300 dark:border-neutral-700 dark:scrollbar-thumb-neutral-700"
      >
        <p class="text-sm font-bold text-neutral-400 dark:text-neutral-500">Roadmaps</p>
      </div>
    </div>
  </div>
</template>
