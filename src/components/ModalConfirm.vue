<script setup lang="ts">
import { ref } from 'vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

const title = ref();
const message = ref();
const okButton = ref();
const cancelButton = ref();

const resolvePromise = ref();
const rejectPromise = ref();

const modalBase = ref();

defineExpose({ show });

function show(opts: any) {
  title.value = opts.title;
  message.value = opts.message;
  okButton.value = opts.okButton;
  cancelButton.value = opts.cancelButton;

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
        <Button @click="_confirm">{{ okButton }}</Button>
        <Button @click="_cancel">{{ cancelButton }}</Button>
      </div>
    </div>
  </ModalBase>
</template>
