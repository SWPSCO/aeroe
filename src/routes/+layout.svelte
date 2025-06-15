<script lang="ts">
  import '../app.css';
  import { mainStore } from '$lib/stores/main';
  import { onMount } from 'svelte';
  import Footer from '$lib/components/Footer.svelte';
  import MinimalTopNav from '$lib/components/shared/TopNav/Minimal.svelte';
  import { page } from '$app/stores';

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
  <div class="flex flex-col h-screen">
    {#if !$page.url.pathname.startsWith('/wallet')}
      <MinimalTopNav />
    {/if}
    <div class="flex flex-col flex-1 min-h-0 overflow-hidden bg-light">
      <slot />
    </div>
    <Footer />
  </div>
{/if} 