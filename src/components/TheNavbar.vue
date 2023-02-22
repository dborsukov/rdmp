<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useGlobalStore } from '@/stores/global';
import IconSun from '@/components/icons/IconSun.vue';
import IconMoon from '@/components/icons/IconMoon.vue';
import IconBarsBottomLeft from '@/components/icons/IconBarsBottomLeft.vue';

const store = useGlobalStore();
const route = useRoute();
</script>

<template>
  <div
    class="flex h-10 w-full shrink-0 items-center gap-x-2 bg-emerald-700 p-2 text-emerald-50 dark:border-b dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <IconBarsBottomLeft class="h-6 cursor-pointer" @click="store.toggleSidebar()" />
    <div class="ml-auto flex items-center gap-x-2">
      <div v-if="route.name == 'roadmap'">[Roadmap Controls]</div>
      <div @click="store.toggleDarkTheme()">
        <IconSun class="h-6 cursor-pointer" v-if="store.darkThemeSelected" />
        <IconMoon class="h-6 cursor-pointer" v-else />
      </div>
    </div>
  </div>
</template>
