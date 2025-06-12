<script lang="ts">
	import { welcomeStore } from '$lib/stores/welcome';
	import Button from '$lib/components/shared/Button.svelte';

	let password = '';
	let confirmPassword = '';
	let loading = false; // We can keep a local loading for the button, but store handles main state

	const handleSubmit = () => {
		if (password.trim() !== confirmPassword.trim()) {
			// This can be a local error shown immediately
			alert('Passwords do not match.');
			return;
		}
		if (password.trim().length < 8) {
			alert('Password must be at least 8 characters long.');
			return;
		}
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
			<input
				type="password"
				bind:value={password}
				placeholder="Password (min. 8 characters)"
				class="p-4 text-lg font-title text-dark border border-dark text-center"
				onkeydown={(e) => {
					if (e.key === 'Enter') handleSubmit();
				}}
			/>
			<input
				type="password"
				bind:value={confirmPassword}
				placeholder="Confirm Password"
				class="p-4 text-lg font-title text-dark border border-dark text-center"
				onkeydown={(e) => {
					if (e.key === 'Enter') handleSubmit();
				}}
			/>
			<Button
				onclick={handleSubmit}
				disabled={loading || password.trim().length < 8 || password.trim() !== confirmPassword.trim()}
			>
				Create Vault
			</Button>
		</div>
	{/if}
</div> 