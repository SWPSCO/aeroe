<script lang="ts">
	import { wallet } from '$lib/scripts/commands';
	import { goto } from '$app/navigation';

	let phrase: string[] = $state(Array(24).fill(''));
	let walletName = $state('Imported Wallet');
	let loading = $state(false);
	let importError: string | null = $state(null);

	const importAndLoadWallet = async () => {
		loading = true;
		importError = null;

		const trimmedPhrase = phrase.map((w) => w.trim());
		if (trimmedPhrase.some((word) => word === '')) {
			importError = 'Please fill in all 24 words of the recovery phrase.';
			loading = false;
			return;
		}

		if (walletName.trim() === '') {
			importError = 'Please enter a name for the wallet.';
			loading = false;
			return;
		}

		const createResult = await wallet.create(walletName, trimmedPhrase);
		if (createResult.success) {
			const loadResult = await wallet.load(walletName);
			if (loadResult.success) {
				goto('/wallet');
			} else {
				importError = `Failed to load imported wallet: ${loadResult.error}`;
			}
		} else {
			importError = `Failed to import wallet: ${createResult.error}`;
		}
		loading = false;
	};
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if loading}
		<div class="animate-pulse text-2xl font-title">Importing Your Wallet...</div>
	{:else}
		<h1 class="text-2xl font-title">Import Wallet</h1>
		<p class="text-md font-title text-center">
			Enter your 24-word recovery phrase and give your wallet a name.
		</p>

		<div class="w-full max-w-sm">
			<input
				type="text"
				bind:value={walletName}
				class="text-lg font-title text-dark border border-dark p-3 text-center w-full focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
						class="text-sm font-title text-dark border border-dark p-2 text-center w-full focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						placeholder={`Word ${index + 1}`}
						aria-label={`Word ${index + 1} of recovery phrase`}
					/>
				</div>
			{/each}
		</div>

		<button
			class="bg-dark text-white py-4 px-8 hover:bg-gray-700 transition-colors duration-150 disabled:opacity-50"
			onclick={importAndLoadWallet}
			disabled={loading || walletName.trim() === ''}
		>
			Import & Open Wallet
		</button>
	{/if}
	{#if importError}
		<div class="text-red-500 max-w-md text-center">{importError}</div>
	{/if}
</div>

<style>
	input {
		min-width: 100px;
	}
</style>