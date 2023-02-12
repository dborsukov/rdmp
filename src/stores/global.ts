import { ref, watch } from 'vue';
import { defineStore } from 'pinia';
import type { Roadmap } from '@/helpers';

export const useGlobalStore = defineStore('global', () => {
  const sidebarExpanded = ref(true);
  function toggleSidebar() {
    sidebarExpanded.value = !sidebarExpanded.value;
  }

  const darkThemeSelected = ref(true);
  function toggleDarkTheme() {
    darkThemeSelected.value = !darkThemeSelected.value;
  }

  watch(darkThemeSelected, () => {
    let html_node = document.getElementsByTagName('html')[0];
    let classlist = html_node.classList;
    if (darkThemeSelected.value) {
      classlist.add('dark');
      return;
    }
    classlist.remove('dark');
  });

  const roadmaps = ref<Roadmap[]>([]);

  return {
    sidebarExpanded,
    toggleSidebar,
    darkThemeSelected,
    toggleDarkTheme,
    roadmaps,
  };
});
