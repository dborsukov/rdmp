<script setup lang="ts">
import { ref } from 'vue';
import { useGlobalStore } from '@/stores/global';
import IconCog from '@/components/icons/IconCog.vue';
import SettingsModal from '@/components/SettingsModal.vue';
import IconBars from '@/components/icons/IconBars.vue';
import VIconButton from '@/components/VIconButton.vue';

const modal = ref<typeof SettingsModal>();

const store = useGlobalStore();
</script>

<template>
  <SettingsModal ref="modal" />
  <div
    class="flex h-10 w-full shrink-0 items-center gap-x-2 border-b border-gray-300 bg-gray-200 p-1 text-gray-500 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <VIconButton @click="store.settings.sidebarExpanded = !store.settings.sidebarExpanded">
      <IconBars class="h-5" />
    </VIconButton>
    <div class="ml-auto flex items-center gap-x-2">
      <VIconButton @click="modal?.open()">
        <IconCog class="h-5" />
      </VIconButton>
    </div>
  </div>
</template>
