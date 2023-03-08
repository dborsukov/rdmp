<script setup lang="ts">
import TheNavbar from '@/components/TheNavbar.vue';
import TheSidebar from '@/components/TheSidebar.vue';
import TheContent from '@/components/TheContent.vue';

document.addEventListener('contextmenu', (event) => event.preventDefault());
document.addEventListener('keydown', (event) => {
  if (event.ctrlKey && (event.key === 'r' || event.key === 'p' || event.key === 'f')) {
    event.preventDefault();
    return;
  }
  if (
    ['F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12'].includes(event.key)
  ) {
    event.preventDefault();
    return;
  }
});
</script>

<template>
  <div class="flex flex-col bg-white text-black dark:bg-neutral-900 dark:text-neutral-50">
    <TheNavbar />
    <div class="flex h-[calc(100vh_-_2.5rem)] w-full flex-grow">
      <TheSidebar />
      <TheContent />
    </div>
  </div>
</template>
