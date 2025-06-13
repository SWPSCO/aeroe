<script lang="ts">
  import '../app.css';
  import { mainStore } from '$lib/stores/main';
  import { onMount } from 'svelte';

  onMount(() => {
    mainStore.boot();
  });
</script>

{#if $mainStore.name === 'booting'}
  <div class="flex justify-center items-center h-screen">
    <div class="animate-pulse text-2xl font-title">Booting...</div>
  </div>
{:else if $mainStore.name === 'error'}
  <div class="m-8">
    <div class="font-title text-xl bg-red-500 text-white p-8">
      <h1 class="font-bold mb-4">A critical error occurred during startup</h1>
      <p class="text-sm">{$mainStore.error}</p>
    </div>
  </div>
{:else}
  <slot />
{/if} 