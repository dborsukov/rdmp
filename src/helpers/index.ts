import { useGlobalStore } from '@/stores/global';
import { invoke } from '@tauri-apps/api/tauri';
import { cloneDeep } from 'lodash';

export type Roadmap = {
  uuid: string;
  title: string;
  description: string;
  nodes: Array<Node>;
};

export type Node = {
  uuid: string;
  title: string;
  description: string;
  isMainNode: boolean;
  children: Array<Node>;
};

export function loadAllRoadmaps() {
  const store = useGlobalStore();
  invoke<Roadmap[]>('load_all_roadmaps')
    .then((result) => {
      store.roadmaps = cloneDeep(result);
    })
    .catch((e) => {
      console.log(e);
    });
}

export function addRoadmap(roadmap: Roadmap) {
  invoke<null>('add_roadmap', { roadmap: roadmap })
    .then(() => {
      loadAllRoadmaps();
    })
    .catch((e) => {
      console.log(e);
    });
}

export function updateRoadmap(roadmap: Roadmap) {
  invoke<null>('update_roadmap', { roadmap: roadmap })
    .then(() => {
      loadAllRoadmaps();
    })
    .catch((e) => {
      console.log(e);
    });
}

export function removeRoadmap(uuid: String) {
  invoke<null>('remove_roadmap', { queryUuid: uuid })
    .then(() => {
      loadAllRoadmaps();
    })
    .catch((e) => {
      console.log(e);
    });
}

export {};
