<script lang="ts">
	import { vault } from '$lib/scripts/commands';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let password = $state('');
	let confirmPassword = $state('');
	let loading = $state(false);
	let error: string | null = $state(null);

	const createAndLoadVault = async () => {
		error = null;
		if (password !== confirmPassword) {
			error = 'Passwords do not match.';
			return;
		}
		if (password.length < 8) {
			error = 'Password must be at least 8 characters long.';
			return;
		}

		loading = true;
		const createResult = await vault.create(password);
		if (createResult.success) {
			const loadResult = await vault.load(password);
			if (loadResult.success) {
				dispatch('created');
			} else {
				error = `Failed to load vault after creation: ${loadResult.error}`;
			}
		} else {
			error = `Failed to create vault: ${createResult.error}`;
		}
		loading = false;
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
			<input
				type="password"
				bind:value={password}
				placeholder="Password (min. 8 characters)"
				class="p-4 text-lg font-title text-dark border border-dark text-center"
				onkeydown={(e) => {
					if (e.key === 'Enter') createAndLoadVault();
				}}
			/>
			<input
				type="password"
				bind:value={confirmPassword}
				placeholder="Confirm Password"
				class="p-4 text-lg font-title text-dark border border-dark text-center"
				onkeydown={(e) => {
					if (e.key === 'Enter') createAndLoadVault();
				}}
			/>
			<button
				class="bg-dark text-white py-4 px-8 disabled:opacity-50"
				onclick={createAndLoadVault}
				disabled={loading || password.length < 8 || password !== confirmPassword}
			>
				Create Vault
			</button>
		</div>
		{#if error}
			<p class="text-red-500 max-w-sm text-center">{error}</p>
		{/if}
	{/if}
</div> 