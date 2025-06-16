<script lang="ts">
	import '../../app.css';
    import { walletStore } from '$lib/stores/wallet';
    import { sessionStore } from '$lib/stores/session';

	import MainArea from '$lib/components/MainArea.svelte';
	import TopNav from '$lib/components/shared/TopNav.svelte';
	import ContentArea from '$lib/components/ContentArea.svelte';
	import FeatureArea from '$lib/components/FeatureArea.svelte';

	let { children } = $props();

    // The $effect rune is global and does not need to be imported.
    // It automatically runs when its dependencies ($sessionStore) change.
    $effect(() => {
        if ($sessionStore.activeWalletName) {
            walletStore.fetchWalletData($sessionStore.activeWalletName);
        } else {
            // If there's no active wallet, we should lock the walletStore
            // to prevent showing stale data.
            walletStore.lock();
        }
    });
</script>

<MainArea>
	{#if $walletStore.status === 'loading' || $walletStore.status === 'locked'}
		<div class="flex justify-center items-center h-screen">
			<div class="animate-pulse text-2xl font-title">Loading Wallet...</div>
		</div>
    {:else if $walletStore.status === 'error'}
        <div class="flex flex-col justify-center items-center h-screen">
			<div class="text-2xl font-title text-red-500">Error Loading Wallet</div>
            <p>{$walletStore.error}</p>
		</div>
	{:else if $walletStore.status === 'loaded'}
		<TopNav />
		<ContentArea class="bg-light">
			<FeatureArea>
				{@render children()}
			</FeatureArea>
		</ContentArea>
	{/if}
</MainArea>