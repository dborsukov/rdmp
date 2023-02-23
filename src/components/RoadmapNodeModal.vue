<script setup lang="ts">
import { ref } from 'vue';
import { v4 } from 'uuid';
import { addNode, updateNode, type Node } from '@/helpers';
import Label from '@/components/VLabel.vue';
import Input from '@/components/VInput.vue';
import Button from '@/components/VButton.vue';
import ModalBase from '@/components/ModalBase.vue';

const emit = defineEmits(['tree-changed']);

const modalBase = ref();
const modalAction = ref();
const incompleteInfo = ref(false);

const modalUuid = ref('');
const modalTitle = ref('');
const modalDescription = ref('');
const modalNodeType = ref('');
const modalDone = ref(false);
const modalSkip = ref(false);
const modalParentNodeUuid = ref<String | null>('');
const modalRoadmapUuid = ref('');

defineExpose({ open });

function open(
  action: 'create' | 'edit',
  existingNode: Node | null,
  nodeType: string,
  parentNodeUuid: string | null,
  roadmapUuid: string
) {
  modalAction.value = action;
  modalNodeType.value = nodeType;
  modalParentNodeUuid.value = parentNodeUuid;
  modalRoadmapUuid.value = roadmapUuid;

  if (existingNode) {
    modalUuid.value = existingNode.uuid;
    modalTitle.value = existingNode.title;
    modalDescription.value = existingNode.description;
    modalNodeType.value = existingNode.nodeType;
    modalDone.value = existingNode.done;
    modalSkip.value = existingNode.skip;
  }

  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
  modalTitle.value = '';
  modalDescription.value = '';
  incompleteInfo.value = false;
}

async function confirm() {
  if (fieldsEmpty()) {
    incompleteInfo.value = true;
    return;
  }
  if (modalAction.value == 'create') {
    await addNode(
      {
        uuid: v4(),
        title: modalTitle.value,
        description: modalDescription.value,
        nodeType: modalNodeType.value,
        done: false,
        skip: false,
        children: [],
      },
      modalParentNodeUuid.value,
      modalRoadmapUuid.value
    ).then(() => {
      emit('tree-changed');
    });
  }
  if (modalAction.value == 'edit') {
    await updateNode({
      uuid: modalUuid.value,
      title: modalTitle.value,
      description: modalDescription.value,
      nodeType: modalNodeType.value,
      done: modalDone.value,
      skip: modalSkip.value,
      children: [],
    }).then(() => {
      emit('tree-changed');
    });
  }
  close();
}

function fieldsEmpty(): Boolean {
  return modalTitle.value.trim() == '' || modalDescription.value.trim() == '';
}
</script>

<template>
  <ModalBase ref="modalBase">
    <div class="flex flex-col gap-y-3">
      <Label>Title</Label>
      <Input v-model="modalTitle" class="w-96" type="text" />
      <Label>Short description</Label>
      <Input v-model="modalDescription" class="w-96" type="text" />
      <p v-if="incompleteInfo" class="text-red-500">Both fields should be filled</p>
      <div class="ml-auto flex gap-x-2">
        <Button v-if="modalAction == 'create'" accent @click="confirm">Create</Button>
        <Button v-if="modalAction == 'edit'" accent @click="confirm">Save</Button>
        <Button @click="close()">Cancel</Button>
      </div>
    </div>
  </ModalBase>
</template>
