<script setup lang="ts">
import { mount } from 'mount-vue-component';
import { ref, watch, onMounted, onBeforeUnmount } from 'vue';
import { loadRoadmap, removeNode, setDone, setSkip, type Node } from '@/helpers';
import RoadmapNode from '@/components/RoadmapNode.vue';
import RoadmapNodeModal from '@/components/RoadmapNodeModal.vue';
import ModalConfirm from '@/components/ModalConfirm.vue';
import ContextMenu from '@/components/ContextMenu.vue';

onMounted(() => {
  render();
  addEventListener('resize', () => {
    redrawSvg(nodes.value);
  });
});

onBeforeUnmount(() => {
  removeEventListener('resize', () => {
    redrawSvg(nodes.value);
  });
});

watch(() => props.roadmapUuid, render);

const props = defineProps({
  roadmapUuid: { type: String, required: true },
});

const nodes = ref<Array<Node>>([]);

const context = ref<typeof ContextMenu>();
const modal = ref<typeof RoadmapNodeModal>();
const modalConfirm = ref<typeof ModalConfirm>();

function showRoadmapNodeModal(
  action: 'create' | 'edit',
  existingNode: Node | null,
  nodeType: String,
  parentNodeUuid: string | null
) {
  modal?.value?.open(action, existingNode, nodeType, parentNodeUuid, props.roadmapUuid);
}

function showContextMenu(event: MouseEvent, item: any, options: Object[]) {
  context?.value?.open(event, item, options);
}
async function handleOption(optionAction: string, node: Node) {
  switch (optionAction) {
    case 'edit': {
      showRoadmapNodeModal('edit', node, node.nodeType, null);
      break;
    }
    case 'setDoneTrue': {
      setDone(node.uuid, true).then(render);
      break;
    }
    case 'setDoneFalse': {
      setDone(node.uuid, false).then(render);
      break;
    }
    case 'setSkipTrue': {
      setSkip(node.uuid, true).then(render);
      break;
    }
    case 'setSkipFalse': {
      setDone(node.uuid, false).then(render);
      break;
    }
    case 'delete': {
      const ok = await modalConfirm?.value?.show({
        title: 'Delete',
        message: `Are you sure you want to delete "${node.title}"?`,
        okButton: 'Delete',
        cancelButton: 'Cancel',
        warning: true,
      });
      if (ok) {
        removeNode(node.uuid).then(render);
        break;
      }
    }
  }
}

async function render() {
  await loadRoadmap(props.roadmapUuid).then((roadmap) => {
    nodes.value = roadmap.nodes;
  });
  let root = document.getElementById('root');
  if (root) {
    root.replaceChildren();
    buildTree(nodes.value, root);
    redrawSvg(nodes.value);
  }
}

function buildTree(nodes: Array<Node>, root_el: HTMLElement) {
  nodes.forEach((node) => {
    let flexWrapper = document.createElement('div');
    flexWrapper.classList.add('my-flex');

    mount(RoadmapNode, {
      props: {
        uuid: node.uuid,
        title: node.title,
        description: node.description,
        nodeType: node.nodeType,
        done: node.done,
        skip: node.skip,
        onCreateNode: showRoadmapNodeModal,
        oncontextmenu: (e: MouseEvent) => {
          if (node.nodeType != 'root') {
            e.preventDefault();
            showContextMenu(e, node, [
              { name: 'Edit', action: 'edit' },
              { name: '|', action: '' },
              { name: 'Done', action: node.done ? 'setDoneFalse' : 'setDoneTrue' },
              { name: 'Skip', action: node.skip ? 'setSkipFalse' : 'setSkipTrue' },
              { name: '|', action: '' },
              { name: 'Delete', action: 'delete' },
            ]);
          }
        },
      },
      element: flexWrapper,
    });

    let childrenFlexColWrapper = document.createElement('div');
    childrenFlexColWrapper.classList.add('my-flex-col');
    flexWrapper.appendChild(childrenFlexColWrapper);

    buildTree(node.children, childrenFlexColWrapper);

    root_el.appendChild(flexWrapper);
  });
}

function redrawSvg(nodes: Array<Node>) {
  let svg = document.getElementById('svg');
  if (svg) {
    svg.setAttribute('width', '0');
    svg.setAttribute('height', '0');
    svg.replaceChildren();
    drawMainNodes(nodes);
    drawChildNodes(nodes);
  }
}

function drawMainNodes(nodes: Array<Node>) {
  nodes.forEach((_, i) => {
    let currentNode = document.querySelector(`[data-node-id='${nodes[i]?.uuid}']`);
    let followingNode = document.querySelector(`[data-node-id='${nodes[i + 1]?.uuid}']`);
    if (followingNode != null && currentNode != null) {
      drawPath(currentNode, followingNode);
    }
  });
}

function drawChildNodes(nodes: Array<Node>) {
  nodes.forEach((node) => {
    node.children.forEach((child) => {
      let parentNode = document.querySelector(`[data-node-id='${node.uuid}']`);
      let childNode = document.querySelector(`[data-node-id='${child.uuid}']`);
      if (parentNode != null && childNode != null) {
        drawPath(parentNode, childNode);
      }
    });
    drawChildNodes(node.children);
  });
}

function drawPath(fromElem: Element, toElem: Element) {
  let svg = document.getElementById('svg');
  let parentRect = document.getElementById('root')?.getBoundingClientRect();

  if (parentRect == undefined || svg == undefined) {
    return;
  }

  let fromRectAbs = fromElem.getBoundingClientRect();
  let toRectAbs = toElem.getBoundingClientRect();

  let fromRect = {
    x: fromRectAbs.x - parentRect.x,
    y: fromRectAbs.y - parentRect.y,
    width: fromRectAbs.width,
    height: fromRectAbs.height,
  };
  let toRect = {
    x: toRectAbs.x - parentRect.x,
    y: toRectAbs.y - parentRect.y,
    width: toRectAbs.width,
    height: toRectAbs.height,
  };

  // @ts-ignore
  if (parseFloat(svg.getAttribute('width')) < toRect.x + toRect.width)
    svg.setAttribute('width', (toRect.x + toRect.width).toString());

  // @ts-ignore
  if (parseFloat(svg.getAttribute('height')) < toRect.y + toRect.height)
    svg.setAttribute('height', (toRect.y + toRect.height).toString());

  if (fromRect.x == toRect.x) {
    let line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    line.classList.add('connection');
    if (fromRect.width < toRect.width) {
      line.setAttribute('x1', (fromRect.x + fromRect.width / 2).toString());
      line.setAttribute('x2', (fromRect.x + fromRect.width / 2).toString());
    } else {
      line.setAttribute('x1', (toRect.x + toRect.width / 2).toString());
      line.setAttribute('x2', (toRect.x + toRect.width / 2).toString());
    }
    line.setAttribute('y1', (fromRect.y + fromRect.height).toString());
    line.setAttribute('y2', toRect.y.toString());
    svg.appendChild(line);
  } else if (fromRect.y == toRect.y) {
    let line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    line.classList.add('connection');
    line.setAttribute('x1', (fromRect.x + fromRect.width).toString());
    line.setAttribute('x2', toRect.x.toString());
    if (fromRect.height < toRect.height) {
      line.setAttribute('y1', (fromRect.y + fromRect.height / 2).toString());
      line.setAttribute('y2', (fromRect.y + fromRect.height / 2).toString());
    } else {
      line.setAttribute('y1', (toRect.y + toRect.height / 2).toString());
      line.setAttribute('y2', (toRect.y + toRect.height / 2).toString());
    }
    svg.appendChild(line);
  } else {
    let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    path.classList.add('connection');

    let startX = fromRect.x + fromRect.width;
    let startY = fromRect.y + fromRect.height / 2;
    let endX = toRect.x;
    let endY = toRect.y + toRect.height / 2;
    let distanceX = endX - startX;
    let distanceY = endY - startY;

    let cpx = startX + distanceX * 0.4;
    let cpy1 = startY;
    let cpy2 = startY + distanceY;

    path.setAttribute(
      'd',
      `M ${startX} ${startY}` +
        `h ${distanceX * 0.2}` +
        `Q ${cpx} ${cpy1} ${cpx} ${startY + distanceY * 0.1}` +
        `v ${distanceY * 0.8}` +
        `Q ${cpx} ${cpy2} ${startX + distanceX * 0.7} ${startY + distanceY}` +
        `h ${distanceX * 0.3}`
    );

    svg.appendChild(path);
  }
}
</script>

<template>
  <ContextMenu ref="context" menu-id="roadmapNodeMenu" @handle-option="handleOption" />
  <ModalConfirm ref="modalConfirm" />
  <RoadmapNodeModal ref="modal" @tree-changed="render" />
  <svg id="svg" class="absolute" width="0" height="0"></svg>
  <main id="root" class="flex h-full w-full flex-col gap-10 p-10"></main>
</template>

<style>
html.dark #svg .connection {
  stroke: #404040;
  stroke-width: 0.1em;
}

#svg .connection {
  stroke: #d4d4d4;
  stroke-width: 0.1em;
}

#svg path {
  fill: none;
}

#root .my-flex {
  gap: 40px;
  display: flex;
  align-items: flex-start;
}

#root .my-flex-col {
  gap: 40px;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
</style>
