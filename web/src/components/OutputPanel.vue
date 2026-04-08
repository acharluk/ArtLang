<script setup lang="ts">
defineProps({
  output: {
    type: String,
    default: "",
  },
  error: {
    type: String,
    default: "",
  },
  success: {
    type: Boolean,
    default: true,
  },
  hasRun: {
    type: Boolean,
    default: false,
  },
});
</script>

<template>
  <div
    class="h-full w-full overflow-auto bg-base-200 p-4 rounded-box font-mono text-sm"
  >
    <!-- placeholder when nothing has run -->
    <p v-if="!hasRun" class="text-base-content/30 italic">
      Run a program to see output here.
    </p>
    <!-- output display -->
    <template v-else>
      <pre
        v-if="output"
        class="whitespace-pre-wrap break-words text-base-content"
        >{{ output }}</pre
      >
      <pre
        v-if="error"
        class="whitespace-pre-wrap break-words text-error font-bold"
        :class="{ 'mt-2': output }"
        >{{ error }}</pre
      >
      <p v-if="success && !output" class="text-base-content/30 italic">
        Program finished with no output.
      </p>
    </template>
  </div>
</template>
