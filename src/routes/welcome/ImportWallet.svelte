<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';
	import { onMount } from 'svelte';

	let phrase: string[] = Array(24).fill('');
	let walletName = 'Imported Wallet';
	let loading = false; // Local loading state for UI feedback

	const handleImport = () => {
		loading = true;
		welcomeStore.importWallet(walletName, phrase);
		// If it fails, the error will be shown on the parent page.
		// We can assume if it continues, it will unmount.
		// To be more robust, we could subscribe to the error state.
		loading = false; 
	}

	function goBack() {
		// Return to the choose action splash screen
		welcomeStore.resetForAdd();
	}
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	<!-- Back button -->
	<button
		class="self-start flex items-center gap-2 px-3 py-1 border-2 border-dark font-title hover:bg-dark hover:text-white transition-colors"
		aria-label="Back"
		on:click={goBack}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
			<path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
		</svg>
		Back
	</button>
	<h1 class="text-2xl font-title">Import Wallet</h1>
	<p class="text-md font-title text-center">
		Enter your 24-word recovery phrase and give your wallet a name.
	</p>

	<div class="w-full max-w-sm">
		<input
			type="text"
			bind:value={walletName}
			class="text-lg font-title text-dark border border-dark p-3 text-center w-full focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
			placeholder="Wallet Name"
			aria-label="Wallet Name"
		/>
	</div>

	<div
		class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4 w-full max-w-4xl px-4"
	>
		{#each { length: 24 } as _, index}
			<div class="flex items-center gap-2">
				<span class="text-sm font-title text-dark w-6 text-right">{index + 1}.</span>
				<input
					type="text"
					bind:value={phrase[index]}
					class="text-sm font-title text-dark border border-dark p-2 text-center w-full focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
					placeholder={`Word ${index + 1}`}
					aria-label={`Word ${index + 1} of recovery phrase`}
				/>
			</div>
		{/each}
	</div>

	<Button
		onclick={handleImport}
		disabled={loading || walletName.trim() === '' || phrase.some(w => w.trim() === '')}
	>
		{loading ? 'Importing...' : 'Import & Open Wallet'}
	</Button>
</div>

<style>
	input {
		min-width: 100px;
	}
</style>