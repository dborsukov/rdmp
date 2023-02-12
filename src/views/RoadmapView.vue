<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useGlobalStore } from '@/stores/global';
import Roadmap from '@/components/RoadmapPage.vue';

const route = useRoute();
const store = useGlobalStore();

const id = ref(route.params.id as string);
const nodes = ref(store.roadmaps.filter((map) => map.id.toString() == id.value)[0].nodes);

watch(
  () => route.params.id,
  () => {
    id.value = route.params.id as string;
    nodes.value = store.roadmaps.filter((map) => map.id.toString() == id.value)[0].nodes;
  }
);
</script>

<template>
  <Roadmap :nodes="nodes" />
</template>
