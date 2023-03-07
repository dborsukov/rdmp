<script setup lang="ts">
import { ref } from 'vue';

const showModal = ref(false);
const showContent = ref(false);

defineExpose({ open, close });

function open() {
  showModal.value = true;
}

function close() {
  showContent.value = false;
}
</script>

<template>
  <transition name="fade" @after-enter="showContent = true">
    <div
      v-if="showModal"
      class="fixed top-0 bottom-0 right-0 left-0 z-50 flex flex-col items-center justify-center bg-black/40"
    >
      <transition name="slide-fade" @after-leave="showModal = false">
        <div
          v-if="showContent"
          class="rounded-lg border border-gray-300 bg-gray-100 p-4 text-gray-700 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
        >
          <slot></slot>
        </div>
      </transition>
    </div>
  </transition>
</template>

<style scoped>
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
.fade-enter-active,
.fade-leave-active {
  transition: 0.1s opacity ease-in-out;
}

.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.1s ease-in-out;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(15px);
  opacity: 0;
}
</style>
