<script setup lang="ts">
import { ref } from 'vue';
import IconPlusCircle from './icons/IconPlusCircle.vue';

const props = defineProps({
  uuid: { type: String, required: true },
  title: { type: String, required: true },
  description: { type: String, required: false },
  nodeType: { type: String, required: true },
});

const hoveringNode = ref(false);
</script>

<template>
  <div class="relative" @mouseover="hoveringNode = true" @mouseleave="hoveringNode = false">
    <IconPlusCircle
      v-if="hoveringNode && (nodeType == 'main' || nodeType == 'root')"
      class="troke-2 absolute -bottom-3 left-1/2 -ml-3 h-6 w-6 cursor-pointer fill-white text-neutral-400 hover:fill-neutral-200 active:-bottom-2 active:-ml-2 active:h-4 active:w-4 dark:fill-neutral-900 dark:hover:fill-neutral-700"
      @click="$emit('createNode', 'create', null, 'main', null)"
    />
    <IconPlusCircle
      v-if="hoveringNode && nodeType != 'root'"
      class="absolute -right-3 top-1/2 -mt-3 h-6 w-6 cursor-pointer fill-white text-neutral-400 hover:fill-neutral-200 active:-right-2 active:-mt-2 active:h-4 active:w-4 dark:fill-neutral-900 dark:hover:fill-neutral-700"
      @click="$emit('createNode', 'create', null, 'child', props.uuid)"
    />
    <div
      :data-node-id="uuid"
      class="rounded-md border-2 border-neutral-300 p-3 dark:border-neutral-700 dark:bg-neutral-800"
    >
      <p class="font-bold">{{ title }}</p>
      <p class="dark:text-neutral-400">{{ description }}</p>
    </div>
  </div>
</template>
