<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useGlobalStore } from '@/stores/global';
import { loadAllRoadmaps, removeRoadmap, type Roadmap } from '@/helpers';
import Label from '@/components/VLabel.vue';
import Link from '@/components/SidebarLink.vue';
import IconPlus from '@/components/icons/IconPlus.vue';
import RoadmapModal from '@/components/RoadmapModal.vue';
import ContextMenu from '@/components/ContextMenu.vue';
import ModalConfirm from '@/components/ModalConfirm.vue';

onMounted(() => {
  loadAllRoadmaps();
  store.readSettingsWrapper();
});

const router = useRouter();
const store = useGlobalStore();

const modal = ref<typeof RoadmapModal>();
const context = ref<typeof ContextMenu>();
const modalConfirm = ref<typeof ModalConfirm>();

function showRoadmapModal(action: 'create' | 'edit', existingRoadmap: Roadmap | null) {
  modal?.value?.open(action, existingRoadmap);
}

function showContextMenu(event: MouseEvent, item: any, options: Object[]) {
  context?.value?.open(event, item, options);
}

const options = [
  { name: 'Edit', action: 'edit' },
  { name: 'Delete', action: 'delete' },
];

async function handleOption(optionAction: string, roadmap: Roadmap) {
  switch (optionAction) {
    case 'edit': {
      showRoadmapModal('edit', roadmap);
      break;
    }
    case 'delete': {
      const ok = await modalConfirm?.value?.show({
        title: 'Delete',
        message: `Are you sure you want to delete "${roadmap.title}"?`,
        okButton: 'Delete',
        cancelButton: 'Cancel',
        warning: true,
      });
      if (ok) {
        router.push('/');
        removeRoadmap(roadmap.uuid);
        break;
      }
    }
  }
}
</script>

<template>
  <ContextMenu ref="context" menu-id="roadmapMenu" @handle-option="handleOption" />
  <ModalConfirm ref="modalConfirm" />
  <RoadmapModal ref="modal" />
  <div
    v-show="store.settings.sidebarExpanded"
    class="flex h-full w-64 flex-shrink-0 flex-col gap-y-2 border-r border-neutral-200 bg-white p-4 text-black dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <Label>Home</Label>
    <Link to="/">Dashboard</Link>
    <Label>Roadmaps</Label>
    <div
      class="flex flex-grow flex-col gap-y-2 overflow-auto rounded-lg border border-neutral-200 p-2 dark:border-neutral-700 dark:bg-neutral-900 dark:text-neutral-50"
    >
      <Link
        v-for="roadmap in store.roadmaps"
        :key="roadmap.uuid"
        :to="`/roadmaps/${roadmap.uuid}`"
        @contextmenu.prevent="showContextMenu($event, roadmap, options)"
        >{{ roadmap.title }}</Link
      >
    </div>
    <button
      class="flex h-10 flex-shrink-0 cursor-pointer items-center justify-center rounded-lg border border-neutral-200 hover:bg-emerald-500/30 active:bg-emerald-500/40 dark:border-neutral-700 dark:hover:bg-neutral-700 dark:active:bg-neutral-900"
      @click="showRoadmapModal('create', null)"
    >
      <IconPlus class="m-auto h-5 w-5" />
    </button>
  </div>
</template>
