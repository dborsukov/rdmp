<template>
  <div
    class="flex aspect-square w-40 flex-col rounded-lg border dark:border-neutral-700 dark:bg-neutral-800"
  >
    <p class="border-b text-center dark:border-neutral-700"><slot name="title" /></p>
    <div class="flex flex-1 items-center justify-center font-mono text-[3rem]">
      <slot name="content" />
    </div>
  </div>
</template>
