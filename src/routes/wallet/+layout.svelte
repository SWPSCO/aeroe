<script>
	import '../../app.css';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { status, get } from '$lib/scripts/wallet-commands';
    import { walletLoaded, walletBalance, walletMasterPubkey } from '$lib/scripts/stores';

	import MainArea from '$lib/components/MainArea.svelte';
	import TopNav from '$lib/components/TopNav.svelte';
	import ContentArea from '$lib/components/ContentArea.svelte';
	import FeatureArea from '$lib/components/FeatureArea.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();

	const checkWalletReady = async () => {
		const res = await status.isReady();
		if (!res.success) {
			setTimeout(checkWalletReady, 1000);
		} else {
			if (res.data) {
				await checkSetupComplete();
			} else {
				setTimeout(checkWalletReady, 1000);
			}
		}
	}

	const checkSetupComplete = async () => {
		const res = await status.isSetupComplete();
		if (!res.success) {
			setTimeout(checkSetupComplete, 1000);
		} else {
			walletLoaded.set(true);
			if (res.data) {
				try {
					const balance = await get.balance();
					walletBalance.set(balance.data);
				} catch (error) {
					walletBalance.set(null);
				}
				try {
					const masterPubkey = await get.masterPubkey();
					walletMasterPubkey.set(masterPubkey.data);
				} catch (error) {
					walletMasterPubkey.set('errored');
				}
				goto('/wallet');
			} else {
				goto('/welcome');
			}
		}
	}

    onMount(async () => {
		await checkWalletReady();
    });

</script>

<MainArea>
	{#if !$walletLoaded}
		<div class="flex justify-center items-center h-screen">
			<div class="animate-pulse text-2xl font-title">Starting <span class="text-highlight-orange">AEROE</span>...</div>
		</div>
	{:else}
		<TopNav />
		<ContentArea>
			<FeatureArea>
				{@render children()}
			</FeatureArea>
		</ContentArea>
		<Footer />
	{/if}
</MainArea>