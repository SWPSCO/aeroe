<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';
	import { sessionStore } from '$lib/stores/session';
	import Modal from '$lib/components/Modal.svelte';

	type LocalStep = 'save' | 'confirm' | 'name' | 'loading';
	let step: LocalStep = 'save';

	let confirmPhrase: string[] = Array(24).fill('');
	let walletName = 'My First Wallet';
	let copied = false; // visual feedback for copy button
	let showSavedModal = false; // controls seed confirmation modal

	// Reactive: check if confirm phrase matches
	let phrasesMatch = false;
	$: phrasesMatch = normalize(confirmPhrase.join(' ')) === ($welcomeStore.seedPhrase ?? []).join(' ').toLowerCase();

	$: nameExists = $sessionStore.wallets.map(n=>n.toLowerCase()).includes(walletName.trim().toLowerCase());

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

	function handleCreate() {
		step = 'loading';
		welcomeStore.createWallet(walletName);
	}
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if !$welcomeStore.seedPhrase}
		<h1 class="text-2xl font-title animate-pulse">Generating Your Secure Keys...</h1>
		<!-- spinning key animation -->
		<svg width="160" height="100" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
			<style>
				:root { --t:3s }
				@keyframes spinKeyY {0%{transform:perspective(600px) rotateY(0deg)}100%{transform:perspective(600px) rotateY(360deg)}}
				.keyGroup{transform-origin:80px 55px; animation:spinKeyY var(--t) linear infinite}
				.head{fill:#000}
				.shaft{fill:#000}
				.tooth{fill:#000}
			</style>
			<g class="keyGroup">
				<rect class="head" x="70" y="60" width="20" height="14" />
				<rect class="shaft" x="74" y="36" width="4" height="24" />

				<rect class="tooth" x="74" y="32" width="16" height="4" />
				<!-- smaller bottom tooth -->
				<rect class="tooth" x="74" y="26" width="10" height="4" />
			</g>
		</svg>
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
			class="self-end mt-2 flex items-center gap-2 px-3 py-2 border-2 font-title transition-colors
				   {copied ? 'bg-dark text-white border-dark' : 'border-dark text-dark'}"
			aria-label="Copy recovery phrase"
			on:click={() => {
				navigator.clipboard.writeText(($welcomeStore.seedPhrase ?? []).join(' '));
				copied = true;
				setTimeout(() => copied = false, 150);
			}}
		>
			<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
				<rect x="4" y="4" width="12" height="12" stroke="currentColor" stroke-width="2" fill="none" />
				<path d="M16 8h4v12H8v-4" stroke="currentColor" stroke-width="2" fill="none" />
			</svg>
			Copy
		</button>
		<Button onclick={() => (showSavedModal = true)}>
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
	{:else if step === 'name'}
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
			{#if nameExists && walletName.trim() !== ''}
				<span class="text-red-500 text-sm">Wallet name already exists</span>
			{/if}
			<Button onclick={handleCreate} disabled={walletName.trim()==='' || nameExists}>
				Create & Open Wallet
			</Button>
		</div>
	{:else if step === 'loading'}
		<div class="flex flex-col items-center gap-4">
			<div class="animate-pulse text-2xl font-title">Opening Your Wallet...</div>
			<svg width="200" height="140" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
				<style>
					:root { --t:3s }
					svg{overflow:visible}
					@keyframes flip {
						0%   { transform:perspective(600px) rotateY(0deg) }
						50%  { transform:perspective(600px) rotateY(180deg) }
						100% { transform:perspective(600px) rotateY(360deg) }
					}
					@keyframes shadow {
						0%,100%{ transform:scaleX(1); opacity:.25 }
						50%    { transform:scaleX(.6) translateX(10px); opacity:.1 }
					}
					@keyframes shine {
						0%   { opacity:0; transform:translateX(-100%) rotateY(-45deg) }
						50%  { opacity:.4 }
						100% { opacity:0; transform:translateX(200%) rotateY(45deg) }
					}
					.card{ fill:url(#grad); rx:8; ry:8; transform-origin:80px 50px; animation:flip var(--t) cubic-bezier(.4,.2,.2,1) infinite }
					.stripe{ fill:#ffffff }
					.shadow{ fill:#000; filter:blur(4px); transform-origin:80px 80px; animation:shadow var(--t) ease-in-out infinite }
					.shine{ fill:#ffffff; mix-blend-mode:overlay; opacity:0; animation:shine var(--t) ease-in-out infinite }
				</style>
				<defs>
					<linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
						<stop offset="0%" stop-color="#000"/>
						<stop offset="100%" stop-color="#222"/>
					</linearGradient>
				</defs>
				<!-- ground shadow -->
				<ellipse class="shadow" cx="80" cy="82" rx="36" ry="8" />

				<!-- card body -->
				<g class="cardGroup">
					<rect class="card" x="40" y="25" width="80" height="50" rx="6" ry="6" />
					<rect class="shine" x="40" y="25" width="80" height="50" rx="6" ry="6" />
				</g>
			</svg>
		</div>
	{/if}

	<!-- Modal asking user to confirm they saved phrase -->
	{#if showSavedModal}
		<Modal open={true} small={true} close={() => (showSavedModal = false)}>
			<div class="h-full flex flex-col items-center justify-center gap-6 p-8 text-center">
				<p class="font-title text-lg max-w-md">I have written down my seed phrase. I understand that it will never be displayed for me again.</p>
				<Button onclick={() => { showSavedModal = false; step = 'name'; }}>
					Confirm
				</Button>
			</div>
		</Modal>
	{/if}
</div>