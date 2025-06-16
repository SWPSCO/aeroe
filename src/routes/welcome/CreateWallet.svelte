<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	type LocalStep = 'save' | 'confirm' | 'name' | 'loading';
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

	function handleCreate() {
		step = 'loading';
		welcomeStore.createWallet(walletName);
	}
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if !$welcomeStore.seedPhrase}
		<h1 class="text-2xl font-title animate-pulse">Generating Your Secure Keys...</h1>
		<svg width="480" height="300" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
		<style>
			/* text drifts up once then idles */
			@keyframes drift{
			0%   {transform:translateY(0)   scale(1);   opacity:1}
			25%  {transform:translateY(-70px) scale(.1); opacity:.05}
			100% {transform:translateY(-70px) scale(.1); opacity:.05}
			}
			.tok{fill:#000;font:600 13px monospace;animation:drift 8s cubic-bezier(.4,0,.2,1) infinite}

			/* verticals grow/shrink in sync from opposite ends */
			@keyframes rise{
			0%   {stroke-dashoffset:40}
			25%  {stroke-dashoffset:0}
			50%  {stroke-dashoffset:0}
			75%  {stroke-dashoffset:40}
			100% {stroke-dashoffset:40}
			}
			.edge{stroke:#000;stroke-width:4;fill:none}
			.vline{stroke-dasharray:40;stroke-dashoffset:40;animation:rise 8s ease-out infinite}

			/* slow fade orange mid-cycle */
			@keyframes spark{
			0%,10%   {fill:#000}
			10%,20%  {fill:#ff4a1a}
			20%,60%  {fill:#000}
			60%,70%  {fill:#ff4a1a}
			70%,100% {fill:#000}
			}
			.key{animation:spark 16s linear infinite}
		</style>

		<!-- drifting text -->
		<text class="tok" x="120" y="88" style="animation-delay:.45s">[2 [0 3] [0 2]]</text>
		<text class="tok" x="18"  y="88">0v1.8afji</text>
		<text class="tok" x="83"  y="88" style="animation-delay:.3s">[2 [2 5] 2 ~]</text>

		<!-- static horizontals & two fixed corner arcs -->
		<path class="edge" d="M60 22 h40"/>
		<path class="edge" d="M60 72 h40"/>
		<path class="edge" d="M60 22 a5 5 0 0 0 -5 5"/>
		<path class="edge" d="M100 72 a5 5 0 0 0 5 -5"/>

		<!-- animated verticals: left grows down, right grows up -->
		<path class="edge vline" d="M55 27 v40"/>
		<path class="edge vline" d="M105 67 v-40"/>

		<!-- key glyph -->
		<g class="key">
			<circle cx="80" cy="35" r="7"/>
			<rect x="77" y="40" width="6" height="18" rx="1"/>
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
			<Button onclick={handleCreate} disabled={walletName.trim()===''}>
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
					<rect class="stripe" x="50" y="40" width="60" height="8" />
					<rect class="shine" x="40" y="25" width="80" height="50" rx="6" ry="6" />
				</g>
			</svg>
		</div>
	{/if}
</div>