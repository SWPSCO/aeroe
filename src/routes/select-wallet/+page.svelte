<script lang="ts">
	import { sessionStore } from '$lib/stores/session';
	import { walletStore } from '$lib/stores/wallet';
	import { mainStore } from '$lib/stores/main';
	import Button from '$lib/components/shared/Button.svelte';
	import Modal from '$lib/components/Modal.svelte';
	
	let loading = false;
	let selectedWallet = '';
	
	async function selectWallet(walletName: string) {
		selectedWallet = walletName;
		loading = true;
		
		await walletStore.loadAndFetch(walletName);
		sessionStore.setActiveWallet(walletName);
		mainStore.authenticate(walletName);
	}
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8 p-8">
	<h1 class="text-2xl font-title">Select a Wallet</h1>
	<p class="text-md font-title text-center max-w-md">
		Choose which wallet you'd like to open.
	</p>
	
	<div class="flex flex-col gap-4 w-full max-w-sm">
		{#if $sessionStore.wallets.length > 0}
			{#each $sessionStore.wallets as wallet}
				<button
					class="text-lg font-title text-dark border-2 border-dark p-4 hover:bg-dark hover:text-white transition-colors"
					on:click={() => selectWallet(wallet)}
					disabled={loading}
				>
					{wallet}
				</button>
			{/each}
		{:else}
			<p class="text-center text-dark">No wallets found</p>
		{/if}
	</div>
</div>

<!-- Loading modal while loading wallet -->
{#if loading}
	<Modal open={true} close={() => {}}>
		<div class="h-full flex flex-col gap-4 items-center justify-center p-8">
			<div class="animate-pulse text-2xl font-title text-center">Loading {selectedWallet}...</div>
			<svg width="160" height="100" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
				<style>
					:root { --t:3s }
					@keyframes flip {0%{transform:perspective(600px) rotateY(0deg)}50%{transform:perspective(600px) rotateY(180deg)}100%{transform:perspective(600px) rotateY(360deg)}}
					@keyframes shadow {0%,100%{transform:scaleX(1);opacity:.25}50%{transform:scaleX(.6) translateX(10px);opacity=.1}}
					.card{fill:url(#grad);rx:8;ry:8;transform-origin:80px 50px;animation:flip var(--t) cubic-bezier(.4,.2,.2,1) infinite}
					.stripe{fill:#ffffff}
					.shadow{fill:#000;filter:blur(4px);transform-origin:80px 80px;animation:shadow var(--t) ease-in-out infinite}
				</style>	
				<defs>
					<linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
						<stop offset="0%" stop-color="#000"/>
						<stop offset="100%" stop-color="#222"/>
					</linearGradient>
				</defs>
				<ellipse class="shadow" cx="80" cy="82" rx="36" ry="8" />
				<rect class="card" x="40" y="25" width="80" height="50" rx="6" ry="6" />
			</svg>
		</div>
	</Modal>
{/if}