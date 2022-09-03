<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window';
</script>

<div data-tauri-drag-region class="h-12  items-center justify-between flex">
  <div />

  <h1
    class="absolute left-1/2 -translate-x-1/2 select-none pointer-events-none text-secondary text-3xl font-extrabold"
  >
    SSH port mapper
  </h1>

  <div>
    <button
      on:click={appWindow.minimize}
      aria-live="polite"
      class="p-2 text-xl hover:text-secondary h-full"
    >
      <i class="fa-solid fa-window-minimize w-6" />
    </button>
    <button
      on:click={appWindow.close}
      aria-live="polite"
      class="p-2 text-2xl hover:text-secondary h-full"
    >
      <i class="fa-solid fa-xmark w-6" />
    </button>
  </div>
</div>
