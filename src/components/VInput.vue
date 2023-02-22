<script setup lang="ts">
defineProps(['modelValue', 'placeholder']);
defineEmits(['update:modelValue']);
</script>

<template>
  <input
    :value="modelValue"
    :placeholder="placeholder"
    @input="$emit('update:modelValue', (<HTMLInputElement>$event.target).value)"
    class="rounded-md border py-1 px-2 focus:outline-none focus:ring-1 focus:ring-neutral-700 dark:border-neutral-700 dark:bg-neutral-800 dark:placeholder-neutral-50/10"
  />
</template>
