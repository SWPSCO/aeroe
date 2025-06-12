<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	let walletName = 'My First Wallet';
	let confirmed = false;
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if !$welcomeStore.seedPhrase}
		<h1 class="text-2xl font-title animate-pulse">Generating Your Secure Keys...</h1>
	{:else if !confirmed}
		<h1 class="text-2xl font-title">Save Your Recovery Phrase</h1>
		<p class="text-md font-title text-center max-w-md">
			Write down these 24 words in order and store them in a secure place.
			<br />
			This is the only way to recover your wallet.
		</p>
		<div class="grid grid-cols-6 gap-4">
			{#each $welcomeStore.seedPhrase as word, index}
				<div class="text-sm font-title text-dark border border-dark p-4 text-center w-[180px]">
					{index + 1}. {word}
				</div>
			{/each}
		</div>
		<Button onclick={() => (confirmed = true)}>
			I Have Saved My Phrase
		</Button>
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
				class="p-2 border border-dark text-center font-title w-full max-w-xs"
			/>
			<Button onclick={() => welcomeStore.createWallet(walletName)}>
				Create & Open Wallet
			</Button>
		</div>
	{/if}
</div>