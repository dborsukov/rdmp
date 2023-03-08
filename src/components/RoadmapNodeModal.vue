<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { v4 } from 'uuid';
import { addNode, updateNode, expandNodesAround, type Node } from '@/helpers';
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

const emit = defineEmits(['tree-changed']);

const modalBase = ref();
const modalAction = ref();
const incompleteTitle = ref(false);

const modalUuid = ref('');
const modalTitle = ref('');
const modalDescription = ref('');
const modalNodeType = ref('');
const modalNodeOrder = ref(0);
const modalDone = ref(false);
const modalSkip = ref(false);
const modalParentNodeUuid = ref<String | null>('');
const modalParentNodeOrder = ref(0);
const modalRoadmapUuid = ref('');

defineExpose({ open });

function open(
  action: 'create' | 'edit',
  existingNode: Node | null,
  nodeType: string,
  parentNodeUuid: string | null,
  parentNodeOrder: number,
  roadmapUuid: string
) {
  modalAction.value = action;
  modalNodeType.value = nodeType;
  modalParentNodeUuid.value = parentNodeUuid;
  modalParentNodeOrder.value = parentNodeOrder;
  modalRoadmapUuid.value = roadmapUuid;

  if (existingNode) {
    modalUuid.value = existingNode.uuid;
    modalTitle.value = existingNode.title;
    modalDescription.value = existingNode.description;
    modalNodeType.value = existingNode.nodeType;
    modalNodeOrder.value = existingNode.nodeOrder;
    modalDone.value = existingNode.done;
    modalSkip.value = existingNode.skip;
  }

  modalBase?.value?.open();
}

function close() {
  modalBase?.value?.close();
  modalTitle.value = '';
  modalDescription.value = '';
  incompleteTitle.value = false;
}

async function confirm() {
  if (modalTitle.value.trim() == '') {
    incompleteTitle.value = true;
    return;
  }
  if (modalAction.value == 'create') {
    let new_node_place = 0;
    if (modalNodeType.value == 'main') {
      new_node_place = modalParentNodeOrder.value + 1;
      expandNodesAround(modalRoadmapUuid.value, new_node_place);
    }
    await addNode(
      {
        uuid: v4(),
        title: modalTitle.value,
        description: modalDescription.value,
        nodeType: modalNodeType.value,
        nodeOrder: new_node_place,
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
      nodeOrder: modalNodeOrder.value,
      done: modalDone.value,
      skip: modalSkip.value,
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
