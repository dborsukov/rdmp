<script setup lang="ts">
import { ref } from 'vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

const title = ref();
const message = ref();
const okButton = ref();
const cancelButton = ref();
const warning = ref();

const resolvePromise = ref();
const rejectPromise = ref();

const modalBase = ref();

defineExpose({ show });

function show(opts: any) {
  title.value = opts.title;
  message.value = opts.message;
  okButton.value = opts.okButton;
  cancelButton.value = opts.cancelButton;
  warning.value = opts.warning;

  modalBase?.value?.open();

  return new Promise((resolve, reject) => {
    resolvePromise.value = resolve;
    rejectPromise.value = reject;
  });
}

function _confirm() {
  modalBase?.value?.close();
  resolvePromise.value(true);
}

function _cancel() {
  modalBase?.value?.close();
  resolvePromise.value(false);
}
</script>

<template>
  <ModalBase ref="modalBase">
    <div class="flex flex-col gap-y-3">
      <p class="font-bold">{{ title }}</p>
      <p>{{ message }}</p>
      <div class="ml-auto flex gap-x-2">
        <button
          v-if="okButton"
          :class="{
            'rounded-md border border-red-600 bg-red-500 px-2 py-1 text-gray-100 hover:bg-red-500 active:bg-red-500 dark:border-red-600 dark:bg-red-500 dark:text-neutral-50 dark:hover:bg-red-500 dark:active:bg-red-500':
              warning,
          }"
          @click="_confirm"
        >
          {{ okButton }}
        </button>
        <Button v-if="cancelButton" @click="_cancel">{{ cancelButton }}</Button>
      </div>
    </div>
  </ModalBase>
</template>
