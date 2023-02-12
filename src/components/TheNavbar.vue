<script setup lang="ts">
import { useUiStore } from '@/stores/ui';
import IconSun from './icons/IconSun.vue';
import IconMoon from './icons/IconMoon.vue';
import IconBarsBottomLeft from './icons/IconBarsBottomLeft.vue';

const ui_store = useUiStore();
</script>

<template>
  <div
    class="flex h-10 w-full shrink-0 items-center gap-x-2 bg-emerald-700 p-2 text-emerald-50 dark:border-b dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <IconBarsBottomLeft class="h-6 cursor-pointer" @click="ui_store.toggleSidebar()" />
    <div class="ml-auto cursor-pointer" @click="ui_store.toggleDarkTheme()">
      <IconSun class="h-6" v-if="ui_store.darkThemeSelected" />
      <IconMoon class="h-6" v-else />
    </div>
  </div>
</template>
