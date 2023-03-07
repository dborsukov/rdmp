<script setup lang="ts">
defineProps({
  modelValue: { type: String, required: true },
});
defineEmits(['update:modelValue']);
</script>

<template>
  <input
    :value="modelValue"
    class="rounded-md border border-gray-300 py-1 px-2 placeholder-gray-700/30 focus:outline-none focus:ring-1 focus:ring-gray-400 dark:border-neutral-700 dark:bg-neutral-800 dark:placeholder-neutral-50/10"
    @input="$emit('update:modelValue', (<HTMLInputElement>$event.target).value)"
  />
</template>
