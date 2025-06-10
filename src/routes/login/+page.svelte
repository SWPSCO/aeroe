<script lang="ts">
	import { vault } from '$lib/scripts/commands';
	import { goto } from '$app/navigation';

	let password = $state('');
	let loading = $state(false);
	let error: string | null = $state(null);

	const unlockVault = async () => {
		error = null;
		loading = true;

		const result = await vault.load(password);
		if (result.success) {
			goto('/wallet');
		} else {
			error = `Failed to unlock vault: ${result.error}`;
		}
		loading = false;
	};
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if loading}
		<div class="animate-pulse text-2xl font-title">Unlocking Vault...</div>
	{:else}
		<h1 class="text-2xl font-title">Unlock Your Vault</h1>
		<p class="text-md font-title text-center max-w-md">
			Enter your local password to access your wallets.
		</p>
		<div class="flex flex-col gap-4 w-full max-w-sm">
			<input
				type="password"
				bind:value={password}
				class="text-lg font-title text-dark border border-dark p-3 text-center w-full focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
				placeholder="Password"
				aria-label="Password"
				onkeydown={(e) => e.key === 'Enter' && unlockVault()}
			/>
			{#if error}
				<p class="text-red-500 text-center">{error}</p>
			{/if}
		</div>
		<button
			class="bg-dark text-white py-4 px-8 hover:bg-gray-700 transition-colors duration-150"
			onclick={unlockVault}
		>
			Unlock
		</button>
	{/if}
</div> 