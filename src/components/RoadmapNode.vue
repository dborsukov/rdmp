<script setup lang="ts">
import { ref } from 'vue';
import IconPlusCircle from '@/components/icons/IconPlusCircle.vue';

defineEmits(['createNode']);

const props = defineProps({
  uuid: { type: String, required: true },
  title: { type: String, required: true },
  description: { type: String, required: true },
  nodeType: { type: String, required: true },
  nodeOrder: { type: Number, required: true },
  done: { type: Boolean, required: true },
  skip: { type: Boolean, required: true },
});

const hoveringNode = ref(false);
</script>

<template>
  <div class="relative" @mouseover="hoveringNode = true" @mouseleave="hoveringNode = false">
    <IconPlusCircle
      v-if="hoveringNode && (nodeType == 'main' || nodeType == 'root')"
      class="absolute -bottom-3 left-1/2 -ml-3 h-6 w-6 cursor-pointer fill-gray-100 text-gray-400 hover:fill-gray-200 dark:fill-neutral-900 dark:hover:fill-neutral-700"
      @click="$emit('createNode', 'create', null, 'main', null, props.nodeOrder)"
    />
    <IconPlusCircle
      v-if="hoveringNode && nodeType != 'root'"
      class="absolute -right-3 top-1/2 -mt-3 h-6 w-6 cursor-pointer fill-gray-100 text-gray-400 hover:fill-gray-200 dark:fill-neutral-900 dark:hover:fill-neutral-700"
      @click="$emit('createNode', 'create', null, 'child', props.uuid, props.nodeOrder)"
    />
    <div
      :data-node-id="uuid"
      :class="{
        'border-dashed bg-transparent dark:bg-transparent': nodeType == 'root',
        'border-gray-300 bg-gray-100 dark:border-neutral-700 dark:bg-neutral-800': !done && !skip,
        'border-emerald-400 bg-emerald-200 dark:border-emerald-400 dark:bg-emerald-800': done,
        'border-dashed border-yellow-500 bg-transparent text-gray-500 dark:border-yellow-400/50 dark:bg-transparent dark:text-neutral-50/30':
          skip,
      }"
      class="max-w-xs cursor-default rounded-md border-2 px-3 py-1"
    >
      <p class="font-bold">{{ title }}</p>
      <p
        v-if="description"
        :class="{
          'text-gray-400 dark:text-neutral-50/80': done,
          'text-gray-400 dark:text-neutral-50/30': skip,
        }"
        class="text-gray-500 dark:text-neutral-50/50"
      >
        {{ description }}
      </p>
    </div>
  </div>
</template>
