<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import CreatePassword from './CreatePassword.svelte';
	import Splash from './Splash.svelte';
	import CreateWallet from './CreateWallet.svelte';
	import ImportWallet from './ImportWallet.svelte';
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if $welcomeStore.step === "createPassword"}
		<CreatePassword />
	{:else if $welcomeStore.step === "chooseAction"}
		<Splash 
			on:create={welcomeStore.chooseCreate}
			on:import={welcomeStore.chooseImport}
		/>
	{:else if $welcomeStore.step === "createWallet"}
		<CreateWallet />
	{:else if $welcomeStore.step === "importWallet"}
		<ImportWallet />
	{/if}

    {#if $welcomeStore.error}
        <p class="text-red-500">{$welcomeStore.error}</p>
    {/if}
</div>