<script lang="ts">
  import { fade } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api';

  let message = '';

  async function callTauri() {
    message = await invoke('tunnel', {
      user: 'ubuntu',
      host: 'jet-black.xyz',
      port: '19999',
      interface: '0.0.0.0'
    });
  }
</script>

<button
  on:click={callTauri}
  class=" font-extrabold btn btn-outline btn-secondary btn-md"
  >Call Tauri</button
>

{message}
