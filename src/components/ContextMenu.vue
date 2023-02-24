<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, nextTick } from 'vue';
import { useClickOutside } from '@/composables';
import IconTrash from '@/components/icons/IconTrash.vue';
import IconForward from '@/components/icons/IconForward.vue';
import IconCheck from '@/components/icons/IconCheck.vue';
import IconEdit from '@/components/icons/IconEdit.vue';
import IconDocumentText from '@/components/icons/IconDocumentText.vue';

onMounted(() => {
  document.body.addEventListener('keyup', onEscPressed);
});

onBeforeUnmount(() => {
  document.body.removeEventListener('keyup', onEscPressed);
});

defineExpose({ open });

const emit = defineEmits(['handle-option']);

const props = defineProps({
  menuId: { type: String, required: true },
});

const active = ref(false);

const menu = ref();
useClickOutside(menu, hide);

const contextItem = ref();
const contextOptions = ref();

async function open(event: MouseEvent, item: any, options: Object[]) {
  contextItem.value = item;
  contextOptions.value = options;

  let menu = document.getElementById(props.menuId);

  if (menu) {
    let finalX = event.screenX;
    let finalY = event.screenY;

    active.value = true;

    await nextTick();

    if (event.screenX + menu.clientWidth > window.window.innerWidth) {
      finalX -= menu.clientWidth;
    }
    if (event.screenY + menu.clientHeight > window.window.innerHeight) {
      finalY -= menu.clientHeight;
    }

    menu.style.left = finalX + 'px';
    menu.style.top = finalY + 'px';
  }
}

function hide() {
  active.value = false;
}

function optionClicked(op: String) {
  emit('handle-option', op, contextItem.value);
  hide();
}

function onEscPressed(event: KeyboardEvent) {
  if (event.code == 'Escape') {
    hide();
  }
}
</script>

<template>
  <Teleport to="body">
    <div v-show="active" :id="menuId" ref="menu" class="absolute min-w-[8rem]">
      <div class="rounded-lg border bg-white shadow-lg dark:border-neutral-700 dark:bg-neutral-800">
        <div v-for="(op, index) in contextOptions" :key="index">
          <hr v-if="op.name == '|'" class="dark:border-neutral-700" />
          <button
            v-else
            class="m-1 w-[calc(100%_-_8px)] rounded-md px-2 py-1 text-left text-sm text-black hover:bg-neutral-200 active:bg-neutral-300 dark:text-neutral-50 dark:hover:bg-neutral-700/70 dark:active:bg-neutral-900/70"
            :class="{
              'hover:text-red-500 dark:hover:text-red-500': op.name == 'Delete',
              'border border-emerald-400 bg-emerald-200 hover:bg-emerald-300 active:bg-emerald-300 dark:border-emerald-400 dark:bg-emerald-800 dark:hover:bg-emerald-700 dark:active:bg-emerald-900':
                contextItem.done && op.name == 'Done',
              'border border-yellow-500 bg-yellow-300 hover:bg-yellow-400 active:bg-yellow-500 dark:border-yellow-400 dark:bg-yellow-600 dark:hover:bg-yellow-500 dark:active:bg-yellow-700':
                contextItem.skip && op.name == 'Skip',
            }"
            @click.stop="optionClicked(op.action)"
          >
            <div class="flex items-center gap-x-2">
              <IconTrash v-if="op.name == 'Delete'" class="h-4" />
              <IconForward v-if="op.name == 'Skip'" class="h-4" />
              <IconCheck v-if="op.name == 'Done'" class="h-4" />
              <IconEdit v-if="op.name == 'Edit'" class="h-4" />
              <IconDocumentText v-if="op.name == 'Details'" class="h-4" />
              {{ op.name }}
            </div>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
