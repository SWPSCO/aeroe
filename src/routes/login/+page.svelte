<script lang="ts">
	import { loginStore } from '$lib/stores/login';
    import Button from '$lib/components/shared/Button.svelte';

	let password = '';
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
	{#if $loginStore.state === 'pending'}
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
				class="text-lg font-title text-dark border border-dark p-3 text-center w-full focus:ring-1 focus:ring-highlight-orange focus:border-highlight-orange"
				placeholder="Password"
				aria-label="Password"
				onkeydown={(e) => e.key === 'Enter' && loginStore.login(password)}
			/>
			{#if $loginStore.state === 'error'}
				<p class="text-red-500 text-center">{$loginStore.error}</p>
			{/if}
		</div>
		<Button onclick={() => loginStore.login(password)}>
			Unlock
		</Button>
	{/if}
</div> 