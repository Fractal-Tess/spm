<script lang="ts">
  import { fade } from 'svelte/transition';

  import Header from '$lib/components/Header.svelte';
  import Footer from '$lib/components/Footer.svelte';
  import background from '$assets/bg.png';
  import InputGroup from '$lib/components/InputGroup.svelte';

  import { ts } from '$lib/stores/tunnelStore';
  import { invoke } from '@tauri-apps/api';

  document.documentElement.setAttribute('data-theme', 'dark');
  document.documentElement.classList.value = 'dark';

  let hasTunneled = true;
  let success = true;
  let message = 'http://localhost:17584/';
  const createTunnel = async () => {
    hasTunneled = true;
    [message, success] = await invoke('create_tunnel', {
      user: $ts.user,
      host: $ts.host,
      port: $ts.port,
      interface: $ts.interface
    });
  };
</script>

<div
  class="font-sans bg-base-100 text-base-content h-screen flex flex-col overflow-y-auto overflow-x-hidden bg-cover bg-bottom"
  style={`background-image:url(${background})`}
  in:fade={{ delay: 300, duration: 1000 }}
>
  <Header />
  <main class="flex-1 flex flex-col items-center justify-center">
    {#if !hasTunneled}
      <form
        on:submit|preventDefault={createTunnel}
        class="flex flex-col items-center"
      >
        <div class="grid p-8 grid-cols-2 grid-rows-2 gap-4 ">
          <InputGroup
            bind:value={$ts.user}
            labelText="user"
            placeholder="root"
          />
          <InputGroup
            bind:value={$ts.host}
            labelText="host"
            placeholder="192.168.0.1"
          />
          <InputGroup
            bind:value={$ts.port}
            labelText="port"
            placeholder="3000"
          />
          <InputGroup
            bind:value={$ts.interface}
            labelText="interface"
            placeholder="localhost"
          />
        </div>
        <div class="text-2xl font-bold">
          <button on:click={createTunnel} class="btn btn-outline btn-secondary">
            Tunnel!
          </button>
        </div>
      </form>
    {:else if success}
      <div
        class="text-2xl font-bold text-center text-base-content select-none pointer-events-none"
      >
        <h1 class="text-3xl">Success!</h1>
        <p>
          Your tunnel should be running on <a
            class="border-b-2 border-secondary text-secondary pointer-events-auto "
            target="_blank"
            href={message}>{message}</a
          >
        </p>
      </div>
    {:else}
      <div class="text-2xl font-bold text-center text-error">
        <h1 class="text-3xl">Failure</h1>
        <p>Sorry, something went wrong and you get no tunnel</p>
      </div>
    {/if}
  </main>
  <Footer />
</div>
