<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useArtLang } from "./composables/useArtLang.ts";
import { examples } from "./data/examples.ts";
import CodeEditor from "./components/CodeEditor.vue";
import OutputPanel from "./components/OutputPanel.vue";

const { wasmReady, wasmError, artlangVersion, loadWasm, run } = useArtLang();

const code = ref(examples[0].code);
const selectedExample = ref(examples[0].id);

const result = ref({ output: "", error: "", success: true });
const hasRun = ref(false);
const isRunning = ref(false);
const elapsed = ref(0);

const splitPercent = ref(50);
const isResizing = ref(false);

const isMac = navigator.platform?.toUpperCase().includes("MAC");
const shortcutLabel = isMac ? "⌘↵" : "Ctrl+↵";

function onSelectExample(event: any) {
  const id = event.target.value;
  const example = examples.find((e: any) => e.id === id);
  if (example) {
    code.value = example.code;
    selectedExample.value = id;
  }
}

function runProgram() {
  if (!wasmReady.value || isRunning.value) return;

  isRunning.value = true;

  requestAnimationFrame(() => {
    setTimeout(() => {
      const res = run(code.value);
      result.value = {
        output: res.output,
        error: res.error,
        success: res.success,
      };
      elapsed.value = res.elapsed;
      hasRun.value = true;
      isRunning.value = false;
    }, 10);
  });
}

function clearOutput() {
  result.value = { output: "", error: "", success: true };
  hasRun.value = false;
  elapsed.value = 0;
}

function startResize(event: any) {
  isResizing.value = true;
  const container = event.target.closest(".panels-container");
  if (!container) return;

  const onMouseMove = (e: any) => {
    const rect = container.getBoundingClientRect();
    const offset = e.clientX - rect.left;
    const pct = (offset / rect.width) * 100;
    splitPercent.value = Math.max(20, Math.min(80, pct));
  };

  const onMouseUp = () => {
    isResizing.value = false;
    window.removeEventListener("mousemove", onMouseMove);
    window.removeEventListener("mouseup", onMouseUp);
  };

  window.addEventListener("mousemove", onMouseMove);
  window.addEventListener("mouseup", onMouseUp);
}

onMounted(() => {
  loadWasm();
});
</script>

<template>
  <!-- Loading overlay -->
  <div
    v-if="!wasmReady && !wasmError"
    class="fixed inset-0 z-50 flex flex-col items-center justify-center gap-4 bg-base-100"
  >
    <span class="loading loading-spinner loading-lg text-primary"></span>
    <p class="text-base-content/60 text-sm">Loading ArtLang...</p>
  </div>

  <div class="flex h-screen flex-col bg-base-100">
    <!-- Navbar -->
    <div class="navbar bg-base-200 px-4 shadow-sm">
      <div class="flex-1 gap-2">
        <div class="flex items-center gap-2">
          <div
            class="flex h-8 w-8 items-center justify-center rounded-lg bg-primary font-bold text-primary-content text-lg"
          >
            A
          </div>
          <span class="text-lg font-bold text-base-content"
            >ArtLang Playground</span
          >
        </div>
        <div v-if="artlangVersion" class="badge badge-ghost badge-sm font-mono">
          v{{ artlangVersion }}
        </div>
      </div>
      <div class="hidden gap-2 sm:flex">
        <a
          href="https://github.com/ACharLuk/ArtLang"
          target="_blank"
          rel="noopener noreferrer"
          class="btn btn-ghost btn-sm"
        >
          GitHub
        </a>
      </div>
    </div>

    <!-- Toolbar -->
    <div
      class="flex flex-wrap items-center gap-2 border-b border-base-300 bg-base-200/50 px-4 py-2"
    >
      <button
        class="btn btn-primary btn-sm gap-1"
        :class="{ 'btn-disabled': !wasmReady || isRunning }"
        :disabled="!wasmReady || isRunning"
        @click="runProgram"
      >
        <span
          v-if="isRunning"
          class="loading loading-spinner loading-xs"
        ></span>
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
          class="h-4 w-4"
        >
          <path
            d="M6.3 2.84A1.5 1.5 0 004 4.11v11.78a1.5 1.5 0 002.3 1.27l9.344-5.891a1.5 1.5 0 000-2.538L6.3 2.841z"
          />
        </svg>
        Run
        <kbd class="kbd kbd-xs hidden sm:inline-flex">{{ shortcutLabel }}</kbd>
      </button>

      <button class="btn btn-ghost btn-sm" @click="clearOutput">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
          class="h-4 w-4"
        >
          <path
            fill-rule="evenodd"
            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
        Clear
      </button>

      <div class="divider divider-horizontal mx-0 hidden sm:flex"></div>

      <select
        class="select select-bordered select-sm w-40"
        :value="selectedExample"
        @change="onSelectExample"
      >
        <option disabled value="">Examples</option>
        <option
          v-for="example in examples"
          :key="example.id"
          :value="example.id"
        >
          {{ example.label }}
        </option>
      </select>

      <div class="flex-1"></div>

      <span
        v-if="hasRun && !isRunning"
        class="text-xs"
        :class="result.success ? 'text-success' : 'text-error'"
      >
        {{ result.success ? "✓" : "✗" }} {{ elapsed.toFixed(1) }}ms
      </span>
    </div>

    <!-- WASM error banner -->
    <div v-if="wasmError" role="alert" class="alert alert-error rounded-none">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5 shrink-0"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 9v2m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <span class="text-sm">
        Failed to load ArtLang WASM module: {{ wasmError }}
        <br />
        <code class="text-xs">Run: bash web/build.sh</code>
      </span>
    </div>

    <!-- Main panels -->
    <div
      class="panels-container flex min-h-0 flex-1 flex-col sm:flex-row"
      :class="{ 'select-none': isResizing }"
    >
      <!-- Editor panel -->
      <div
        class="flex min-h-0 flex-col sm:min-w-0"
        :style="{ flex: `0 0 ${splitPercent}%` }"
      >
        <div
          class="flex items-center gap-2 border-b border-base-300 px-4 py-1.5"
        >
          <span
            class="text-xs font-semibold uppercase tracking-wider text-base-content/50"
            >Editor</span
          >
        </div>
        <div class="min-h-0 flex-1">
          <CodeEditor v-model="code" @run="runProgram" />
        </div>
      </div>

      <!-- Resize handle -->
      <div
        class="hidden cursor-col-resize items-center justify-center bg-base-300 transition-colors hover:bg-primary/20 sm:flex"
        :class="isResizing ? 'w-1 bg-primary/40' : 'w-1'"
        @mousedown="startResize"
      ></div>

      <!-- Output panel -->
      <div class="flex min-h-0 flex-1 flex-col sm:min-w-0">
        <div
          class="flex items-center gap-2 border-b border-base-300 px-4 py-1.5"
        >
          <span
            class="text-xs font-semibold uppercase tracking-wider text-base-content/50"
            >Output</span
          >
        </div>
        <div class="min-h-0 flex-1">
          <OutputPanel
            :output="result.output"
            :error="result.error"
            :success="result.success"
            :has-run="hasRun"
          />
        </div>
      </div>
    </div>
  </div>
</template>
