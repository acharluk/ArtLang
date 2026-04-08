<script setup lang="ts">
import { ref, computed, nextTick } from "vue";

const props = defineProps({
  modelValue: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["update:modelValue", "run"]);

const gutterRef: any = ref(null);
const editorRef: any = ref(null);

const lineCount = computed(() => {
  const text = props.modelValue || "";
  return text.split("\n").length;
});

const lineNumbers = computed(() => {
  return Array.from({ length: lineCount.value }, (_, i) => i + 1);
});

function onInput(event: any) {
  emit("update:modelValue", event.target.value);
}

function syncScroll() {
  if (gutterRef.value && editorRef.value) {
    gutterRef.value.scrollTop = editorRef.value.scrollTop;
  }
}

function handleKeydown(event: any) {
  // Ctrl+Enter or Cmd+Enter → emit run
  if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
    event.preventDefault();
    emit("run");
    return;
  }

  // Tab → insert 4 spaces
  if (event.key === "Tab" && !event.shiftKey) {
    event.preventDefault();
    const textarea: any = editorRef.value;
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const value = props.modelValue || "";
    const newValue = value.substring(0, start) + "    " + value.substring(end);
    emit("update:modelValue", newValue);
    nextTick(() => {
      textarea.selectionStart = start + 4;
      textarea.selectionEnd = start + 4;
    });
    return;
  }

  // Shift+Tab → dedent current line (remove up to 4 leading spaces)
  if (event.key === "Tab" && event.shiftKey) {
    event.preventDefault();
    const textarea = editorRef.value;
    const start = textarea.selectionStart;
    const value = props.modelValue || "";

    // Find the start of the current line
    const lineStart = value.lastIndexOf("\n", start - 1) + 1;
    const lineEnd = value.indexOf("\n", start);
    const currentLine = value.substring(
      lineStart,
      lineEnd === -1 ? value.length : lineEnd,
    );

    // Count leading spaces to remove (up to 4)
    let spacesToRemove = 0;
    for (let i = 0; i < Math.min(4, currentLine.length); i++) {
      if (currentLine[i] === " ") {
        spacesToRemove++;
      } else {
        break;
      }
    }

    if (spacesToRemove > 0) {
      const newValue =
        value.substring(0, lineStart) +
        currentLine.substring(spacesToRemove) +
        value.substring(lineEnd === -1 ? value.length : lineEnd);
      emit("update:modelValue", newValue);
      const newCursor = Math.max(lineStart, start - spacesToRemove);
      nextTick(() => {
        textarea.selectionStart = newCursor;
        textarea.selectionEnd = newCursor;
      });
    }
    return;
  }
}
</script>

<template>
  <div class="flex h-full w-full overflow-hidden rounded-box">
    <!-- Line numbers gutter -->
    <div
      ref="gutterRef"
      class="bg-base-300 text-base-content/40 flex flex-col items-end overflow-hidden py-3 pr-3 pl-2 font-mono text-sm leading-6 select-none"
    >
      <div v-for="lineNum in lineNumbers" :key="lineNum">{{ lineNum }}</div>
    </div>
    <!-- Editor textarea -->
    <textarea
      ref="editorRef"
      :value="modelValue"
      @input="onInput"
      @scroll="syncScroll"
      @keydown="handleKeydown"
      class="bg-base-200 text-base-content flex-1 resize-none border-none py-3 pl-3 pr-4 font-mono text-sm leading-6 outline-none"
      spellcheck="false"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      placeholder="Write your ArtLang code here..."
    />
  </div>
</template>
