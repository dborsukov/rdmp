<script setup lang="ts">
import { ref } from 'vue';
import { v4 } from 'uuid';
import { addRoadmap, updateRoadmap, type Roadmap } from '@/helpers';
import Label from '@/components/VLabel.vue';
import Input from '@/components/VInput.vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

const modalBase = ref();
const type = ref('create');
const incompleteInfo = ref(false);

const uuid = ref('');
const title = ref('');
const description = ref('');

defineExpose({ open });

function open(type_: string, roadmap: Roadmap | null) {
  type.value = type_;
  if (roadmap) {
    uuid.value = roadmap.uuid;
    title.value = roadmap.title;
    description.value = roadmap.description;
  }
  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
  title.value = '';
  description.value = '';
  incompleteInfo.value = false;
}

function confirm() {
  if (title.value.trim() == '' || description.value.trim() == '') {
    incompleteInfo.value = true;
    return;
  }
  if (type.value == 'create') {
    addRoadmap({
      uuid: v4(),
      title: title.value,
      description: description.value,
      nodes: [],
    });
  }
  if (type.value == 'edit') {
    updateRoadmap({
      uuid: uuid.value,
      title: title.value,
      description: description.value,
      nodes: [],
    });
  }
  close();
}
</script>

<template>
  <ModalBase ref="modalBase">
    <div class="flex flex-col gap-y-3">
      <Label>Title</Label>
      <Input class="w-96" type="text" v-model="title" />
      <Label>Short description</Label>
      <Input class="w-96" type="text" v-model="description" />
      <p class="text-red-500" v-if="incompleteInfo">Both fields should be filled</p>
      <div class="ml-auto flex gap-x-2">
        <Button v-if="type == 'create'" accent @click="confirm">Create</Button>
        <Button v-if="type == 'edit'" accent @click="confirm">Save</Button>
        <Button @click="close()">Cancel</Button>
      </div>
    </div>
  </ModalBase>
</template>
