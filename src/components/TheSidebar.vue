<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/api/dialog';
import { useGlobalStore } from '@/stores/global';
import {
  type Roadmap,
  loadAllRoadmaps,
  removeRoadmap,
  importRoadmap,
  exportRoadmap,
} from '@/helpers';
import Label from '@/components/VLabel.vue';
import Link from '@/components/SidebarLink.vue';
import IconPlus from '@/components/icons/IconPlus.vue';
import RoadmapModal from '@/components/RoadmapModal.vue';
import ContextMenu from '@/components/ContextMenu.vue';
import ModalConfirm from '@/components/ModalConfirm.vue';
import IconArrowDownOnSquare from '@/components/icons/IconArrowDownOnSquare.vue';

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
  { name: 'Export', action: 'export' },
  { name: 'Delete', action: 'delete' },
];

async function handleOption(optionAction: string, roadmap: Roadmap) {
  switch (optionAction) {
    case 'edit': {
      showRoadmapModal('edit', roadmap);
      break;
    }
    case 'export': {
      const selected_folder: any = await open({
        directory: true,
        multiple: false,
      });
      if (selected_folder != null) {
        exportRoadmap(roadmap.uuid, roadmap.title, selected_folder)
          .then(async () => {
            await modalConfirm?.value?.show({
              title: 'Export',
              message: 'Export finished successfuly',
              okButton: 'Got it',
            });
          })
          .catch(async (err) => {
            await modalConfirm?.value?.show({
              title: 'Export',
              message: `Export failed: ${err}`,
              okButton: 'Got it',
            });
          });
      }
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

async function importRoadmapWrapper() {
  const selected_file = await open({
    directory: false,
    multiple: false,
    filters: [
      {
        name: 'Roadmap',
        extensions: ['rdmp'],
      },
    ],
  });
  if (!selected_file) return;
  importRoadmap(selected_file as string)
    .then(async (new_map) => {
      await modalConfirm?.value?.show({
        title: 'Import',
        message: 'Import finished successfuly',
        okButton: 'Got it',
      });
      loadAllRoadmaps();
      router.push(`/roadmaps/${new_map.uuid}`);
    })
    .catch(async (err) => {
      await modalConfirm?.value?.show({
        title: 'Import',
        message: `Import failed: ${err}`,
        okButton: 'Got it',
      });
    });
}
</script>

<template>
  <ContextMenu ref="context" menu-id="roadmapMenu" @handle-option="handleOption" />
  <ModalConfirm ref="modalConfirm" />
  <RoadmapModal ref="modal" />
  <div
    v-show="store.settings.sidebarExpanded"
    class="flex h-full w-64 flex-shrink-0 flex-col gap-y-2 border-r border-gray-300 bg-gray-200 p-4 text-gray-800 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-50"
  >
    <Label>Home</Label>
    <Link to="/">Dashboard</Link>
    <Label>Roadmaps</Label>
    <div
      class="flex flex-grow flex-col gap-y-2 overflow-auto rounded-lg border border-gray-300 bg-gray-50 p-2 dark:border-neutral-700 dark:bg-neutral-900 dark:text-neutral-50"
    >
      <transition-group name="roadmaps-list">
        <Link
          v-for="roadmap in store.roadmaps"
          :key="roadmap.uuid"
          :to="`/roadmaps/${roadmap.uuid}`"
          @contextmenu.prevent="showContextMenu($event, roadmap, options)"
          >{{ roadmap.title }}</Link
        >
      </transition-group>
    </div>
    <div class="flex flex-shrink-0 gap-2">
      <button
        class="flex aspect-square h-10 cursor-pointer items-center justify-center rounded-lg border border-gray-300 hover:bg-gray-400/20 active:bg-gray-400/40 dark:border-neutral-700 dark:hover:bg-neutral-700 dark:active:bg-neutral-900"
        @click="importRoadmapWrapper()"
      >
        <IconArrowDownOnSquare class="h-5 w-5" />
      </button>
      <button
        class="flex h-10 flex-1 cursor-pointer items-center justify-center rounded-lg border border-gray-300 hover:bg-gray-400/20 active:bg-gray-400/40 dark:border-neutral-700 dark:hover:bg-neutral-700 dark:active:bg-neutral-900"
        @click="showRoadmapModal('create', null)"
      >
        <IconPlus class="h-5 w-5" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.roadmaps-list-move,
.roadmaps-list-enter-active,
.roadmaps-list-leave-active {
  transition: all 0.3s ease;
}

.roadmaps-list-leave-active {
  position: absolute;
}

.roadmaps-list-enter-from {
  opacity: 0;
  transform: translateY(15px);
}

.roadmaps-list-leave-to {
  opacity: 0;
  transform: scale(0);
}
</style>
