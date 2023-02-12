<script setup lang="ts">
import TheNavbar from '@/components/TheNavbar.vue';
import TheSidebar from '@/components/TheSidebar.vue';
import TheContent from '@/components/TheContent.vue';

document.addEventListener('contextmenu', (event) => event.preventDefault());
</script>

<template>
  <div class="flex h-full flex-col bg-white text-black dark:bg-neutral-900 dark:text-neutral-50">
    <TheNavbar />
    <div class="flex w-full flex-grow">
      <TheSidebar />
      <TheContent />
    </div>
  </div>
</template>
