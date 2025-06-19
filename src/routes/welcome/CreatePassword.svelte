<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	let password = '';
	let confirmPassword = '';
	let loading = false; // We can keep a local loading for the button, but store handles main state

	// Reactive validation flags
	$: mismatch = confirmPassword !== '' && password.trim() !== confirmPassword.trim();
	$: tooShort = password.trim().length > 0 && password.trim().length < 8;

	// Password strength indicator (0-3)
	$: level = password.trim().length >= 8 ? 3 : password.trim().length >= 3 ? 2 : password.trim().length > 0 ? 1 : 0;

	// Determine colors for the three bar segments
	$: barColors = (() => {
		const base = ['bg-gray-300', 'bg-gray-300', 'bg-gray-300'];
		if (level === 1) base[0] = 'bg-red-500';
		else if (level === 2) base[0] = base[1] = 'bg-orange-500';
		else if (level === 3) base[0] = base[1] = base[2] = 'bg-green-500';
		return base;
	})();

	const handleSubmit = () => {
		// Basic validation; UI already indicates issues
		if (mismatch || tooShort) return;
		loading = true;
		setTimeout(() => welcomeStore.submitPassword(password), 50);
		// No need to set loading = false, the component will be unmounted on success
	};
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if loading}
		<div class="animate-pulse text-2xl font-title mb-4">Creating Your Encrypted Vault...</div>
		<!-- Animated key-generation graphic (same as CreateWallet) -->
		<svg width="480" height="300" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
			<style>
				@keyframes drift{0%{transform:translateY(0) scale(1);opacity:1}25%{transform:translateY(-70px) scale(.1);opacity:.05}100%{transform:translateY(-70px) scale(.1);opacity:.05}}
				.tok{fill:#000;font:600 13px monospace;animation:drift 8s cubic-bezier(.4,0,.2,1) infinite}
				@keyframes rise{0%{stroke-dashoffset:40}25%{stroke-dashoffset:0}50%{stroke-dashoffset:0}75%{stroke-dashoffset:40}100%{stroke-dashoffset:40}}
				.edge{stroke:#000;stroke-width:4;fill:none}
				.vline{stroke-dasharray:40;stroke-dashoffset:40;animation:rise 8s ease-out infinite}
				@keyframes spark{0%,10%{fill:#000}10%,20%{fill:#ff4a1a}20%,60%{fill:#000}60%,70%{fill:#ff4a1a}70%,100%{fill:#000}}
				.key{animation:spark 16s linear infinite}
			</style>
			<text class="tok" x="120" y="88" style="animation-delay:.45s">[2 [0 3] [0 2]]</text>
			<text class="tok" x="18"  y="88">0v1.8afji</text>
			<text class="tok" x="83"  y="88" style="animation-delay:.3s">[2 [2 5] 2 ~]</text>
			<path class="edge" d="M60 22 h40"/>
			<path class="edge" d="M60 72 h40"/>
			<path class="edge" d="M60 22 a5 5 0 0 0 -5 5"/>
			<path class="edge" d="M100 72 a5 5 0 0 0 5 -5"/>
			<path class="edge vline" d="M55 27 v40"/>
			<path class="edge vline" d="M105 67 v-40"/>
			<g class="key"><circle cx="80" cy="35" r="7"/><rect x="77" y="40" width="6" height="18" rx="1"/></g>
		</svg>
	{:else}
		<h1 class="text-2xl font-title">Create a Local Password</h1>
		<p class="text-md font-title text-center max-w-md">
			This password encrypts your wallet data on this device.
			<br />
			It cannot be recovered, so store it safely.
		</p>
		<div class="flex flex-col gap-4 w-full max-w-sm">
			<div class="relative w-full inline-block">
				<input
					type="password"
					bind:value={password}
					placeholder="Password"
					class="w-full p-4 text-lg font-title text-dark border border-dark text-center focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
					onkeydown={(e) => {
						if (e.key === 'Enter') handleSubmit();
					}}
				/>
				{#if password.trim().length > 0}
					<div class="absolute left-full ml-2 top-1/2 -translate-y-1/2 flex">
						{#each barColors as c}
							<div class={`w-2 h-1 ${c}`}></div>
						{/each}
					</div>
				{/if}
			</div>
			<div class="relative w-full inline-block">
				<input
					type="password"
					bind:value={confirmPassword}
					placeholder="Confirm Password"
					class="w-full p-4 text-lg font-title text-dark border border-dark text-center focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
					onkeydown={(e) => {
						if (e.key === 'Enter') handleSubmit();
					}}
				/>
				{#if mismatch}
					<span class="absolute left-full ml-2 top-1/2 -translate-y-1/2 text-red-500 text-sm whitespace-nowrap">*Passwords do not match</span>
				{/if}
			</div>
			<Button
				onclick={handleSubmit}
				disabled={loading || password.trim().length < 8 || password.trim() !== confirmPassword.trim()}
			>
				Create Vault
			</Button>
		</div>
	{/if}
</div> 