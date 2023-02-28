<script setup lang="ts">
import { ref } from 'vue';
import { useGlobalStore } from '@/stores/global';
import ModalBase from '@/components/ModalBase.vue';
import Toggle from '@/components/VToggle.vue';

defineExpose({ open });

const store = useGlobalStore();
const modalBase = ref<typeof ModalBase>();

function open() {
  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
}
</script>

<template>
  <ModalBase ref="modalBase">
    <div class="flex min-h-[10rem] min-w-[20rem] flex-col gap-y-3">
      <p
        class="cursor-pointer text-neutral-900/30 hover:underline dark:text-neutral-500"
        @click="close()"
      >
        Close
      </p>
      <div class="flex">
        <p>Dark theme</p>
        <Toggle v-model="store.settings.darkTheme" class="ml-auto" />
      </div>
      <div class="flex">
        <p>Show numbers next to nodes</p>
        <Toggle v-model="store.settings.showNumbers" class="ml-auto" />
      </div>
    </div>
  </ModalBase>
</template>
