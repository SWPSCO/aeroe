<script lang="ts">
	import '../../app.css';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { aeroe, wallet } from '$lib/scripts/commands';
    import { walletLoaded, walletBalance, walletMasterPubkey } from '$lib/scripts/stores';

	import MainArea from '$lib/components/MainArea.svelte';
	import TopNav from '$lib/components/TopNav.svelte';
	import ContentArea from '$lib/components/ContentArea.svelte';
	import FeatureArea from '$lib/components/FeatureArea.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
	let retryCount = 0;
	const MAX_RETRIES = 5;

	const checkWalletStatus = async () => {
		try {
			const statusRes = await aeroe.status();
			if (!statusRes.success) {
				if (retryCount < MAX_RETRIES) {
					retryCount++;
					setTimeout(checkWalletStatus, 2000);
				} else {
					console.error('Failed to get aeroe status after retries:', statusRes.error);
				}
				return;
			}

			const status = statusRes.data;
			
			if (!status.vault_exists) {
				goto('/'); // Go to landing page to start the whole flow.
				return;
			}
			
			if (!status.vault_loaded) {
				goto('/login'); // If vault exists but isn't loaded, prompt for password.
				return;
			}

			if (status.wallets.length === 0) {
				goto('/welcome'); // No wallets created yet.
				return;
			}

			// If we reach here, the vault is loaded and there's at least one wallet.
			walletLoaded.set(true);

			if (status.active_wallet) {
				await loadWalletData(status.active_wallet);
			} else {
                // If there's no active wallet, this layout shouldn't be active.
                // Or we should show a wallet selection screen.
                // For now, let's assume the router directs away from here if no active_wallet.
				console.log("No active wallet is set.");
			}

		} catch (error) {
			console.error('Error checking wallet status:', error);
			if (retryCount < MAX_RETRIES) {
				retryCount++;
				setTimeout(checkWalletStatus, 2000);
			}
		}
	};

	const loadWalletData = async (walletName: string) => {
		try {
			const balanceRes = await wallet.balance(walletName);
			if (balanceRes.success) {
				walletBalance.set(balanceRes.data);
			} else {
				console.warn('Failed to load wallet balance:', balanceRes.error);
			}
		} catch (error) {
			console.warn('Error loading wallet balance:', error);
		}

		try {
			const pubkeyRes = await wallet.masterPubkey(walletName);
			if (pubkeyRes.success) {
				walletMasterPubkey.set(pubkeyRes.data);
			} else {
				console.warn('Failed to load master pubkey:', pubkeyRes.error);
				walletMasterPubkey.set('error');
			}
		} catch (error) {
			console.warn('Error loading master pubkey:', error);
			walletMasterPubkey.set('error');
		}
	};

    onMount(async () => {
		await checkWalletStatus();
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