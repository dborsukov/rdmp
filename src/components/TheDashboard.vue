<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getRoadmapsAmount, getNodesAmount } from '@/helpers';
import Box from '@/components/DashboardBox.vue';
import Section from '@/components/DashboardSection.vue';

const roadmapsAmount = ref(0);
const nodesAmount = ref(0);

onMounted(() => {
  getRoadmapsAmount().then((amount) => {
    roadmapsAmount.value = amount;
  });
  getNodesAmount().then((amount) => {
    nodesAmount.value = amount;
  });
});
</script>

<template>
  <div class="flex h-full justify-center p-10">
    <div class="flex h-full w-3/4 max-w-5xl flex-col gap-4">
      <Section>
        <template #title>Overview</template>
        <template #content>
          <div class="flex flex-wrap gap-4">
            <Box>
              <template #title>Roadmaps</template>
              <template #content>{{ roadmapsAmount }}</template>
            </Box>
            <Box>
              <template #title>Nodes</template>
              <template #content>{{ nodesAmount }}</template>
            </Box>
          </div>
        </template>
      </Section>
    </div>
  </div>
</template>
