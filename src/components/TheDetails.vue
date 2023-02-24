<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { marked } from 'marked';
import hljs from 'highlight.js';
import autosize from 'autosize';
import { useGlobalStore } from '@/stores/global';
import { type Node, loadDetails, saveDetails } from '@/helpers';
import IconPencil from '@/components/icons/IconPencil.vue';
import IconEye from '@/components/icons/IconEye.vue';

onMounted(() => {
  const editor = editorArea.value;
  if (editor) {
    autosize(editor);
    editor.focus();
    editor.addEventListener('keydown', (event) => {
      if (event.key == 'Tab') {
        event.preventDefault();
        let start = editor.selectionStart;
        let end = editor.selectionEnd;
        editor.value = editor.value.substring(0, start) + '\t' + editor.value.substring(end);
        editor.selectionStart = editor.selectionEnd = start + 1;
      }
    });
    editor.addEventListener('keyup', () => {
      if (detailsNode.value) {
        saveDetails(detailsNode.value.uuid, markdown.value);
      }
    });
  }
  addEventListener('keyup', handleKeys);
});

onBeforeUnmount(() => {
  removeEventListener('keyup', handleKeys);
});

defineExpose({ show });

const emit = defineEmits(['close']);

const markdown = ref();
const detailsNode = ref<Node>();
const store = useGlobalStore();
const previewActive = ref(false);
const previewDiv = ref<HTMLDivElement>();
const editorArea = ref<HTMLTextAreaElement>();

function show(node: Node) {
  detailsNode.value = node;
  loadDetails(node.uuid).then(async (result) => {
    markdown.value = result;
    await nextTick();
    const editor = editorArea.value;
    if (editor) {
      autosize.update(editor);
    }
    if (markdown.value) {
      togglePreview();
    }
  });
}

function handleKeys(event: KeyboardEvent) {
  if (event.code == 'Escape') {
    emit('close');
  }
  if (event.ctrlKey && event.key === 'p') {
    togglePreview();
  }
}

function togglePreview() {
  if (!previewActive.value) {
    if (previewDiv.value) {
      previewDiv.value.innerHTML = marked.parse(markdown.value);
    }
    hljs.highlightAll();
  }
  previewActive.value = !previewActive.value;
}
</script>

<template>
  <link
    rel="stylesheet"
    href="/src/assets/stackoverflow-dark.css"
    :disabled="!store.darkThemeSelected"
  />
  <link
    rel="stylesheet"
    href="/src/assets/stackoverflow-light.css"
    :disabled="store.darkThemeSelected"
  />
  <div class="relative flex h-full w-full justify-center px-14 py-6 dark:bg-neutral-900">
    <div class="flex w-3/4 max-w-5xl flex-col gap-y-3">
      <div class="flex items-center">
        <p
          class="cursor-pointer text-neutral-900/30 hover:underline dark:text-neutral-500"
          @click="$emit('close')"
        >
          Go Back
        </p>
        <div
          class="ml-auto cursor-pointer text-neutral-900/30 hover:text-neutral-900 dark:text-neutral-50/50 dark:hover:text-neutral-50/80"
          @click="togglePreview()"
        >
          <IconPencil v-if="previewActive" class="h-5" />
          <IconEye v-else class="h-5" />
        </div>
      </div>
      <p class="text-2xl font-bold dark:text-neutral-50">{{ detailsNode?.title }}</p>
      <p class="text-xl text-neutral-900/50 dark:text-neutral-50/70">
        {{ detailsNode?.description }}
      </p>
      <hr class="border-t border-dashed dark:border-neutral-700" />
      <div class="relative mb-6 select-auto">
        <div v-show="previewActive" id="preview" ref="previewDiv"></div>
        <textarea
          v-show="!previewActive"
          ref="editorArea"
          v-model="markdown"
          autofocus
          placeholder="Use markdown to add some details to this node..."
          rows="1"
          class="w-full resize-none bg-transparent font-mono text-neutral-900 dark:text-neutral-50"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<style>
#preview h1 {
  font-size: 1.5rem;
  line-height: 2rem;
  margin-bottom: 1rem;
}
#preview h2 {
  font-size: 1.25rem;
  line-height: 1.75rem;
  margin-bottom: 1rem;
}
#preview h3 {
  font-size: 1.125rem;
  line-height: 1.75rem;
  margin-bottom: 1rem;
}
#preview ul {
  margin-left: 1rem;
  margin-bottom: 1rem;
  list-style-type: disc;
  list-style-position: inside;
}
#preview ol {
  margin-left: 1rem;
  margin-bottom: 1rem;
  list-style-type: decimal;
  list-style-position: inside;
}
#preview code {
  margin: 1rem 0;
  border: 1px solid #171717;
}
html.dark #preview code {
  margin: 1rem 0;
  border: 1px solid #404040;
}
</style>
