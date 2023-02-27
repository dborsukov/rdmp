<script setup lang="ts">
import Toggle from '@vueform/toggle';
</script>

<template>
  <Toggle
    :classes="{
      container: 'inline-block rounded-full outline-none focus:none',
      toggle:
        'flex w-8 h-4 rounded-md relative cursor-pointer transition items-center box-content border-2 text-xs leading-none',
      toggleOn:
        'bg-emerald-600 border-emerald-600 dark:bg-emerald-600 dark:border-emerald-600 justify-start text-white',
      toggleOff:
        'bg-neutral-400 border-neutral-400 dark:bg-neutral-400 dark:border-neutral-400 justify-end text-gray-700',
      toggleOnDisabled:
        'bg-gray-300 border-gray-300 justify-start text-gray-400 cursor-not-allowed',
      toggleOffDisabled: 'bg-gray-200 border-gray-200 justify-end text-gray-400 cursor-not-allowed',
      handle: 'inline-block bg-white w-4 h-4 top-0 rounded absolute transition-all',
      handleOn: 'left-full transform -translate-x-full',
      handleOff: 'left-0',
      handleOnDisabled: 'bg-gray-100 left-full transform -translate-x-full',
      handleOffDisabled: 'bg-gray-100 left-0',
      label: 'text-center w-8 border-box whitespace-nowrap select-none',
    }"
  />
</template>
