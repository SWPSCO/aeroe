<script lang="ts">
  import { onMount } from 'svelte';
  import { welcomeStore } from '$lib/stores/welcome';
  import Splash from '../../routes/welcome/Splash.svelte';
  import CreateWallet from '../../routes/welcome/CreateWallet.svelte';
  import ImportWallet from '../../routes/welcome/ImportWallet.svelte';

  onMount(() => {
    welcomeStore.resetForAdd();
  });
</script>

<div class="p-4 flex flex-col gap-6 items-center justify-center h-full">
  {#if $welcomeStore.step === 'chooseAction'}
    <Splash showHeader={false} on:create={welcomeStore.chooseCreate} on:import={welcomeStore.chooseImport} />
  {:else if $welcomeStore.step === 'createWallet'}
    <CreateWallet />
  {:else if $welcomeStore.step === 'importWallet'}
    <ImportWallet />
  {:else if $welcomeStore.step === 'finished'}
    <!-- finished handled by parent -->
  {/if}

  {#if $welcomeStore.error}
    <p class="text-red-500">{$welcomeStore.error}</p>
  {/if}
</div> 