<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	let walletName = 'My First Wallet';
	let confirmed = false;
</script>

<div class="flex flex-col gap-8 items-center justify-center">
	{#if !$welcomeStore.seedPhrase}
		<h1 class="text-2xl font-title animate-pulse">Generating your keys...</h1>
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
			I have recorded my phrase
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