<script setup lang="ts">
import { ref } from 'vue';
import { useGlobalStore } from '@/stores/global';
import IconCog from '@/components/icons/IconCog.vue';
import SettingsModal from '@/components/SettingsModal.vue';
import IconBarsBottomLeft from '@/components/icons/IconBarsBottomLeft.vue';

const modal = ref<typeof SettingsModal>();

const store = useGlobalStore();
</script>

<template>
  <SettingsModal ref="modal" />
  <div
    class="flex h-10 w-full shrink-0 items-center gap-x-2 bg-emerald-700 p-2 text-emerald-50 dark:border-b dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <IconBarsBottomLeft
      class="h-6 cursor-pointer"
      @click="store.settings.sidebarExpanded = !store.settings.sidebarExpanded"
    />
    <div class="ml-auto flex items-center gap-x-2">
      <IconCog class="h-6 cursor-pointer" @click="modal?.open()" />
    </div>
  </div>
</template>
