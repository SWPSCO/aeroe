<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	type LocalStep = 'save' | 'confirm' | 'name';
	let step: LocalStep = 'save';

	let confirmPhrase: string[] = Array(24).fill('');
	let walletName = 'My First Wallet';

	// Reactive: check if confirm phrase matches
	let phrasesMatch = false;
	$: phrasesMatch = normalize(confirmPhrase.join(' ')) === ($welcomeStore.seedPhrase ?? []).join(' ').toLowerCase();

	function normalize(str: string) {
		return str.trim().replace(/\s+/g, ' ').toLowerCase();
	}

	function handlePaste(event: ClipboardEvent) {
		event.preventDefault();
		const text = event.clipboardData?.getData('text') ?? '';
		const words = normalize(text).split(' ');
		if (words.length >= 24) {
			confirmPhrase = words.slice(0,24);
		}
	}
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if !$welcomeStore.seedPhrase}
		<h1 class="text-2xl font-title animate-pulse">Generating Your Secure Keys...</h1>
	{:else if step === 'save'}
		<h1 class="text-2xl font-title">Save Your Recovery Phrase</h1>
		<p class="text-md font-title text-center max-w-md">
			Write down these 24 words in order and store them in a secure place.
			<br />
			This is the only way to recover your wallet.
		</p>
		<div class="w-full max-h-[60vh] overflow-y-auto p-2">
			<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
				{#each $welcomeStore.seedPhrase as word, index}
					<div class="text-sm font-title text-dark border border-dark p-4 text-center w-full min-w-[120px]">
						{index + 1}. {word}
					</div>
				{/each}
			</div>
		</div>

		<!-- Copy to clipboard button -->
		<button
			class="self-end mt-2 flex items-center gap-2 px-3 py-2 border-2 border-dark font-title"
			aria-label="Copy recovery phrase"
			on:click={() => navigator.clipboard.writeText(($welcomeStore.seedPhrase ?? []).join(' '))}
		>
			<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
				<rect x="4" y="4" width="12" height="12" stroke="currentColor" stroke-width="2" fill="none" />
				<path d="M16 8h4v12H8v-4" stroke="currentColor" stroke-width="2" fill="none" />
			</svg>
			Copy
		</button>
		<Button onclick={() => (step = 'confirm')}>
			I Have Saved My Phrase
		</Button>
	{:else if step === 'confirm'}
		<h1 class="text-2xl font-title">Confirm Your Recovery Phrase</h1>

		<div class="w-full max-h-[60vh] overflow-y-auto p-2" on:paste={handlePaste}>
			<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
				{#each { length: 24 } as _, index}
					<input
						type="text"
						bind:value={confirmPhrase[index]}
						class="text-sm font-title text-dark border border-dark p-4 text-center w-full min-w-[120px] focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
						placeholder={`${index + 1}`}
					/>
				{/each}
			</div>
		</div>

		{#if !phrasesMatch && confirmPhrase.some(w => w.trim() !== '')}
			<p class="text-red-500 text-sm">Phrase does not match.</p>
		{/if}

		<Button onclick={() => (step = 'name')} disabled={!phrasesMatch}>Continue</Button>
	{:else}
		<h1 class="text-2xl font-title">Name Your Wallet</h1>
		<p class="text-md font-title text-center max-w-md">
			Give your new wallet a name to easily identify it.
		</p>
		<div class="flex flex-col gap-4 items-center">
			<input
				type="text"
				bind:value={walletName}
				placeholder="Wallet Name"
				class="p-2 border border-dark text-center font-title w-full max-w-xs focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
			/>
			<Button onclick={() => welcomeStore.createWallet(walletName)}>
				Create & Open Wallet
			</Button>
		</div>
	{/if}
</div>