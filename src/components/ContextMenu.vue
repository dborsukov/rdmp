<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from 'vue';
import { useClickOutside } from '@/composables';

const props = defineProps({
  menuId: { type: String, required: true },
  options: { type: Array<String>, required: true },
});

const emit = defineEmits(['handle-option']);

defineExpose({ open });

const active = ref(false);

const item = ref();

const menu = ref();

useClickOutside(menu, hide);

function open(event: MouseEvent, it: any) {
  item.value = it;

  let menu = document.getElementById(props.menuId);
  if (!menu) return;

  menu.style.left = event.screenX + 'px';
  menu.style.top = event.screenY + 'px';

  active.value = true;
}

function hide() {
  const menu = document.getElementById(props.menuId);
  if (menu) {
    active.value = false;
  }
}

function optionClicked(op: String) {
  emit('handle-option', op, item.value);
  hide();
}

function onEscPressed(event: KeyboardEvent) {
  if (event.code == 'Escape') {
    hide();
  }
}

onMounted(() => {
  document.body.addEventListener('keyup', onEscPressed);
});

onBeforeUnmount(() => {
  document.body.removeEventListener('keyup', onEscPressed);
});
</script>

<template>
  <Teleport to="body">
    <div :id="menuId" ref="menu" v-show="active" class="absolute z-50 w-40">
      <div
        class="flex flex-col gap-y-2 rounded-lg border bg-white p-2 shadow-lg dark:border-neutral-700 dark:bg-neutral-800"
      >
        <div v-for="(op, index) in options" :key="index" @click.stop="optionClicked(op)">
          <button
            class="w-full rounded-md px-2 py-1 text-left text-sm text-black hover:bg-neutral-200 active:bg-neutral-300 dark:text-neutral-50 dark:hover:bg-neutral-700 dark:active:bg-neutral-900"
          >
            {{ op }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
