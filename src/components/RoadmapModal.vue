<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { v4 } from 'uuid';
import { useRouter } from 'vue-router';
import { addNode, addRoadmap, updateRoadmap, loadAllRoadmaps, type Roadmap } from '@/helpers';
import Label from '@/components/VLabel.vue';
import Input from '@/components/VInput.vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

onMounted(() => {
  addEventListener('keyup', handleKeys);
});

function handleKeys(event: KeyboardEvent) {
  if (event.code == 'Escape') {
    close();
  }
}

const router = useRouter();

const modalBase = ref();
const modalAction = ref();
const incompleteTitle = ref(false);

const modalUuid = ref('');
const modalTitle = ref('');
const modalDescription = ref('');

defineExpose({ open });

function open(action: 'create' | 'edit', existingRoadmap: Roadmap | null) {
  modalAction.value = action;
  if (existingRoadmap) {
    modalUuid.value = existingRoadmap.uuid;
    modalTitle.value = existingRoadmap.title;
    modalDescription.value = existingRoadmap.description;
  }
  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
  modalTitle.value = '';
  modalDescription.value = '';
  incompleteTitle.value = false;
}

function confirm() {
  if (modalTitle.value.trim() == '') {
    incompleteTitle.value = true;
    return;
  }
  if (modalAction.value == 'create') {
    let newRoadmapUuid = v4();
    addRoadmap({
      uuid: newRoadmapUuid,
      title: modalTitle.value,
      description: modalDescription.value,
      nodes: [],
    })
      .then(() => {
        addNode(
          {
            uuid: v4(),
            title: 'Start',
            description: '',
            nodeType: 'root',
            nodeOrder: 0,
            done: false,
            skip: false,
            children: [],
          },
          null,
          newRoadmapUuid
        );
      })
      .then(() => {
        loadAllRoadmaps();
        router.push(`/roadmaps/${newRoadmapUuid}`);
      });
  }
  if (modalAction.value == 'edit') {
    updateRoadmap({
      uuid: modalUuid.value,
      title: modalTitle.value,
      description: modalDescription.value,
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
      <Input
        v-model="modalTitle"
        :class="{
          'border-red-500 dark:border-red-500': incompleteTitle,
        }"
        class="w-96"
        type="text"
        @keyup.enter="confirm"
      />
      <p v-if="incompleteTitle" class="text-red-500">Title cannot be empty</p>
      <Label>Short description</Label>
      <Input v-model="modalDescription" class="w-96" type="text" @keyup.enter="confirm" />
      <div class="ml-auto flex gap-x-2">
        <Button v-if="modalAction == 'create'" accent @click="confirm">Create</Button>
        <Button v-if="modalAction == 'edit'" accent @click="confirm">Save</Button>
        <Button @click="close()">Cancel</Button>
      </div>
    </div>
  </ModalBase>
</template>
