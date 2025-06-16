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
		welcomeStore.submitPassword(password);
		// No need to set loading = false, the component will be unmounted on success
	};
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if loading}
		<div class="animate-pulse text-2xl font-title">Creating Your Encrypted Vault...</div>
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