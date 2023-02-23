<script setup lang="ts">
import { ref } from 'vue';
import { v4 } from 'uuid';
import type { Node } from '@/helpers';
import { addNode, updateNode } from '@/helpers';
import Label from '@/components/VLabel.vue';
import Input from '@/components/VInput.vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

const emit = defineEmits(['tree-changed']);

const type = ref('');
const modalBase = ref();
const incompleteInfo = ref(false);

const uuid = ref('');
const title = ref('');
const description = ref('');
const nodeType_ = ref('');
const parentNode = ref<String | null>('');
const roadmap = ref('');

defineExpose({ open });

function open(
  modalType: string,
  node: Node | null,
  nodeType: string,
  parentNodeUUID: string | null,
  roadmapUUID: string
) {
  type.value = modalType;
  parentNode.value = parentNodeUUID;
  roadmap.value = roadmapUUID;
  nodeType_.value = nodeType;
  if (node) {
    uuid.value = node.uuid;
    title.value = node.title;
    description.value = node.description;
    nodeType_.value = node.nodeType;
  }
  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
  title.value = '';
  description.value = '';
  incompleteInfo.value = false;
}

async function confirm() {
  if (title.value.trim() == '' || description.value.trim() == '') {
    incompleteInfo.value = true;
    return;
  }
  if (type.value == 'create') {
    console.log('created node.');
    await addNode(
      {
        uuid: v4(),
        title: title.value,
        description: description.value,
        nodeType: nodeType_.value,
        children: [],
      },
      parentNode.value,
      roadmap.value
    ).then(() => {
      emit('tree-changed');
    });
  }
  if (type.value == 'edit') {
    console.log('edited node.');
    await updateNode({
      uuid: uuid.value,
      title: title.value,
      description: description.value,
      nodeType: nodeType_.value,
      children: [],
    }).then(() => {
      emit('tree-changed');
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
