<script lang="ts">
	import { onMount } from 'svelte';
	import { wallet } from '$lib/scripts/commands';
	import { goto } from '$app/navigation';

	let phrase: string[] = $state([]);
	let walletName = $state('My First Wallet');
	let loading = $state(false);
	let error: string | null = $state(null);
	let step = $state(1); // 1: Show phrase, 2: Confirm and create

	const handleKeygen = async () => {
		const result = await wallet.keygen();
		if (result.success) {
			phrase = result.data;
		} else {
			error = `Failed to generate keys: ${result.error}`;
		}
	};

	const createAndLoadWallet = async () => {
		loading = true;
		error = null;
		const createResult = await wallet.create(walletName.trim(), phrase);
		if (createResult.success) {
			const loadResult = await wallet.load(walletName.trim());
			if (loadResult.success) {
				goto('/wallet');
			} else {
				error = `Failed to load new wallet: ${loadResult.error}`;
			}
		} else {
			error = `Failed to create wallet: ${createResult.error}`;
		}
		loading = false;
	};

	onMount(async () => {
		await handleKeygen();
	});
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if phrase.length === 0 && !error}
		<h1 class="text-2xl font-title animate-pulse">Generating Your Secure Keys...</h1>
	{:else if error}
		<h1 class="text-md font-title text-red-500">
			An error occurred: {error}. Please try again.
		</h1>
	{:else}
		{#if step === 1}
			<h1 class="text-2xl font-title">Save Your Recovery Phrase</h1>
			<p class="text-md font-title text-center max-w-md">
				Write down these 24 words in order and store them in a secure place.
				<br />
				This is the only way to recover your wallet.
			</p>
			<div class="grid grid-cols-6 gap-4">
				{#each phrase as word, index}
					<div class="text-sm font-title text-dark border border-dark p-4 text-center w-[180px]">
						{index + 1}. {word}
					</div>
				{/each}
			</div>
			<button class="bg-dark text-white py-4 px-8" onclick={() => (step = 2)}>
				I Have Saved My Phrase
			</button>
		{:else if step === 2}
			<h1 class="text-2xl font-title">Name Your Wallet</h1>
			<p class="text-md font-title text-center max-w-md">
				Give your new wallet a name to easily identify it.
			</p>
			<div class="flex flex-col gap-4 items-center">
				<input
					type="text"
					bind:value={walletName}
					placeholder="Wallet Name"
					class="p-2 border border-dark text-center font-title w-full max-w-xs"
				/>
				<button
					class="bg-dark text-white py-4 px-8 disabled:opacity-50"
					onclick={createAndLoadWallet}
					disabled={loading}
				>
					{loading ? 'Creating Wallet...' : 'Create & Open Wallet'}
				</button>
				{#if error}
					<p class="text-red-500 text-center">{error}</p>
				{/if}
			</div>
		{/if}
	{/if}
</div>