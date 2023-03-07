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
        class="cursor-pointer text-gray-400 hover:underline dark:text-neutral-500"
        @click="close()"
      >
        Close
      </p>
      <div class="flex items-center">
        <div>
          <p>Dark theme</p>
          <p class="text-sm text-gray-500 dark:text-neutral-500">Use dark application colors</p>
        </div>
        <Toggle v-model="store.settings.darkTheme" class="ml-auto" />
      </div>
      <hr class="border-gray-300 dark:border-neutral-700" />
      <div class="flex items-center">
        <div>
          <p>Stage numbers</p>
          <p class="text-sm text-gray-500 dark:text-neutral-500">
            Show numbers next to roadmap stages
          </p>
        </div>
        <Toggle v-model="store.settings.showNumbers" class="ml-auto" />
      </div>
    </div>
  </ModalBase>
</template>
