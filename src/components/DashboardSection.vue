<template>
  <div class="flex flex-col gap-4">
    <p class="text-2xl font-bold text-gray-400 dark:text-neutral-500"><slot name="title" /></p>
    <div class="flex">
      <slot name="content" />
    </div>
  </div>
</template>
