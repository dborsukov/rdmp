import { ref, watch } from 'vue';
import { defineStore } from 'pinia';
import { readSettings } from '@/helpers';
import { type Roadmap, writeSettings } from '@/helpers';

export const useGlobalStore = defineStore('global', () => {
  const roadmaps = ref<Roadmap[]>([]);

  const settings = ref<any>({
    darkTheme: true,
    sidebarExpanded: true,
  });

  function readSettingsWrapper() {
    readSettings().then((r) => {
      settings.value.darkTheme = r.darkTheme != undefined ? r.darkTheme : true;
      settings.value.sidebarExpanded = r.sidebarExpanded != undefined ? r.sidebarExpanded : true;
    });
  }

  watch(settings.value, () => {
    writeSettings();
  });

  watch(
    () => settings.value.darkTheme,
    () => {
      let html_node = document.getElementsByTagName('html')[0];
      let classlist = html_node.classList;
      if (settings.value.darkTheme) {
        classlist.add('dark');
      } else {
        classlist.remove('dark');
      }
    }
  );

  return {
    roadmaps,
    settings,
    readSettingsWrapper,
  };
});
