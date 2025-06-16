<script lang="ts">
    import { walletStore } from '$lib/stores/wallet';
    import { sessionStore } from '$lib/stores/session';
    import TransactionList from '$lib/components/wallet/TransactionList.svelte';
    import SendForm from '$lib/components/wallet/SendForm.svelte';
    import QrCode from '$lib/components/wallet/QrCode.svelte';
    import Modal from '$lib/components/Modal.svelte';
    import AddWalletWizard from '$lib/components/AddWalletWizard.svelte';
    import { welcomeStore } from '$lib/stores/welcome';

	let showSendForm = true;
	let showReceive = false;
	let showWalletMenu = false;
	let showAddWalletModal = false;

	$: activeWalletName = $sessionStore.activeWalletName;


	// Close modal automatically once wizard signals finished
	$: if (showAddWalletModal && $welcomeStore.step === 'finished') {
		showAddWalletModal = false;
	}
</script>

<div class="w-full flex flex-col gap-8 p-8">
	{#if $walletStore.balance}
		<div class="px-8 pt-8 pb-4">
			<!-- Wallet identity -->
			<div class="flex items-center justify-between bg-gray-1 p-4 {showWalletMenu ? '' : 'mb-6'}">
				<div class="flex flex-col">
					<div class="font-title text-lg text-dark">{activeWalletName}</div>
					{#if $walletStore.masterPubkey}
						<div class="text-xs font-mono text-dark break-all max-w-full text-left">{$walletStore.masterPubkey}</div>
					{/if}
				</div>
				<!-- Hamburger menu icon -->
				<button aria-label="Wallet menu" class="ml-4" on:click={() => showWalletMenu = !showWalletMenu}>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-dark" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
					</svg>
				</button>
			</div>

			{#if showWalletMenu}
				<div class="bg-gray-1 w-full mb-6">
					{#each $sessionStore.wallets.filter(w => w !== activeWalletName) as w}
						<button class="w-full px-4 py-3 flex items-center justify-between border-t first:border-t-0 hover:bg-gray-200"
							on:click={() => { sessionStore.setActiveWallet(w); walletStore.fetchWalletData(w); showWalletMenu = false; }}>
							<span class="font-title text-dark">{w}</span>
						</button>
					{/each}

					<button class="w-full px-4 py-3 flex items-center justify-between border-t hover:bg-gray-200"
						on:click={() => { welcomeStore.resetForAdd(); showAddWalletModal = true; showWalletMenu = false; }}>
						<span class="font-title text-dark">Add Wallet</span>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-dark" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v14m7-7H5" />
						</svg>
					</button>
				</div>
			{/if}

			<div class=" font-title text-dark text-sm">Current Balance</div>
			<div class="flex items-end gap-2 mt-2">
				<div class="text-4xl text-highlight-orange font-bold">
					{$walletStore.balance.amount}
				</div>
				<div class="uppercase text-2xl text-dark font-title">
					{$walletStore.balance.coin}
				</div>
			</div>
			<div class="flex gap-4 mt-6">
				<button
					on:click={() => {
						showSendForm = true;
						showReceive = false;
					}}
					class="flex-1 p-4 bg-dark text-white font-title"
				>
					Send
				</button>
				<button
					on:click={() => {
						showReceive = true;
						showSendForm = false;
					}}
					class="flex-1 p-4 border-2 border-dark font-title"
				>
					Receive
				</button>
            </div>
        </div>
    {/if}

	{#if showSendForm}
		<div class="px-8">
			<SendForm />
		</div>
	{/if}

	{#if showReceive}
		<div class="px-8 flex flex-col items-center">
			<h2 class="font-title text-xl mb-1">Your Address</h2>

			{#if $walletStore.masterPubkey}
				<!-- Address text directly under the heading -->
				<div class="p-2 bg-light break-all font-mono w-full max-w-lg text-center">
					{$walletStore.masterPubkey}
				</div>

				<!-- QR code -->
				<div class="mt-4">
					<QrCode address={$walletStore.masterPubkey} />
				</div>

				<!-- Copy icon button in place where address used to be -->
				<button class="mt-4" aria-label="Copy address"
					on:click={() => $walletStore.masterPubkey && navigator.clipboard.writeText($walletStore.masterPubkey)}>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-dark" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<rect x="4" y="4" width="12" height="12" stroke-width="2" stroke="currentColor" fill="none"/>
						<!-- Bottom-right square with missing top-left corner (so it appears behind) -->
						<path d="M16 8 H20 V20 H8 V16" stroke-width="2" stroke="currentColor" fill="none" />
					</svg>
				</button>
			{:else}
				<p class="text-red-500">Could not load wallet address.</p>
			{/if}
		</div>
	{/if}

	<div class="px-8 pb-8">
		<TransactionList transactions={$walletStore.transactions} />
	</div>
</div>

{#if showAddWalletModal}
  <Modal open={true} close={() => showAddWalletModal = false}>
    <AddWalletWizard />
  </Modal>
{/if} 