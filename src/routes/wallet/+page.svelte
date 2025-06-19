<script lang="ts">
    import { walletStore } from '$lib/stores/wallet';
    import { sessionStore } from '$lib/stores/session';
    import TransactionList from '$lib/components/wallet/TransactionList.svelte';
    import SendForm from '$lib/components/wallet/SendForm.svelte';
    import QrCode from '$lib/components/wallet/QrCode.svelte';
    import Modal from '$lib/components/Modal.svelte';
    import AddWalletWizard from '$lib/components/AddWalletWizard.svelte';
    import { welcomeStore } from '$lib/stores/welcome';
    import MinimalNav from '$lib/components/shared/TopNav/Minimal.svelte';

	let showSendForm = true;
	let showReceive = false;
	let showWalletMenu = false;
	let showAddWalletModal = false;

	// Visual feedback state for copy address
	let copiedAddress = false;

	$: activeWalletName = $sessionStore.activeWalletName;


	// Close modal automatically once wizard signals finished
	$: if (showAddWalletModal && $welcomeStore.step === 'finished') {
		showAddWalletModal = false;
	}
</script>

<div class="w-full">
	<div class="max-w-6xl mx-auto flex flex-col gap-8 p-8">
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
							on:click={() => {
								walletStore.loadAndFetch(w);
								sessionStore.setActiveWallet(w);
								showWalletMenu = false;
							}}>
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
					{$walletStore.balance}
				</div>
				<div class="uppercase text-2xl text-dark font-title">
					Nock
				</div>
			</div>
			<div class="flex gap-4 mt-6">
				<button
					on:click={() => {
						showSendForm = true;
						showReceive = false;
					}}
					class={`flex-1 p-4 font-title border-2 border-dark transition-colors ${showSendForm ? 'bg-dark text-white' : 'bg-white text-dark'}`}
				>
					Send
				</button>
				<button
					on:click={() => {
						showReceive = true;
						showSendForm = false;
					}}
					class={`flex-1 p-4 font-title border-2 border-dark transition-colors ${showReceive ? 'bg-dark text-white' : 'bg-white text-dark'}`}
				>
					Receive
				</button>
				</div>
			</div>
		{:else}
			<div class="px-8 pt-8 pb-4 animate-pulse font-title">Loading balance…</div>
		{/if}

		{#if showSendForm}
			<div class="px-8">
				<SendForm />
			</div>
		{/if}

		{#if showReceive}
			<div class="px-8">
				<div class="flex flex-col gap-4 p-6 border-2 border-dark bg-white items-center">
				<h2 class="font-title text-xl">Your Address</h2>

				{#if $walletStore.masterPubkey}
					<!-- Address text -->
					<div class="pt-0 pb-2 px-2 break-all font-mono w-full max-w-lg text-center">
						{$walletStore.masterPubkey}
					</div>

					<!-- QR code -->
					<div class="w-[225px] h-[225px] flex items-center justify-center">
						<QrCode address={$walletStore.masterPubkey} />
					</div>

					<!-- Copy icon button with feedback -->
					<button
						class="mt-2 p-2 transition-colors {copiedAddress ? 'bg-dark' : 'border-dark'}"
						aria-label="Copy address"
						on:click={() => {
							if ($walletStore.masterPubkey) {
								navigator.clipboard.writeText($walletStore.masterPubkey);
								copiedAddress = true;
								setTimeout(() => copiedAddress = false, 150);
							}
						}}
					>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 {copiedAddress ? 'text-white' : 'text-dark'}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<rect x="4" y="4" width="12" height="12" stroke-width="2" stroke="currentColor" fill="none"/>
							<path d="M16 8 H20 V20 H8 V16" stroke-width="2" stroke="currentColor" fill="none" />
						</svg>
					</button>
				{:else}
					<p class="text-red-500">Could not load wallet address.</p>
				{/if}
				</div>
			</div>
		{/if}

		<div class="px-8 pb-8">
			<TransactionList transactions={$walletStore.transactions} />
		</div>
	</div>
</div>

{#if showAddWalletModal}
  <Modal open={true} close={() => { if ($welcomeStore.step === 'chooseAction' || $welcomeStore.step === 'importWallet') showAddWalletModal = false; }}>
    <AddWalletWizard />
  </Modal>
{/if}

<!-- Loading modal while switching wallets -->
{#if $walletStore.fetching}
  <Modal open={true} close={() => {}}>
    <div class="h-full flex flex-col gap-4 items-center justify-center p-8">
		<div class="animate-pulse text-2xl font-title text-center">Loading Wallet...</div>
      <svg width="160" height="100" viewBox="0 0 160 100" xmlns="http://www.w3.org/2000/svg">
        <style>
          :root { --t:3s }
          @keyframes flip {0%{transform:perspective(600px) rotateY(0deg)}50%{transform:perspective(600px) rotateY(180deg)}100%{transform:perspective(600px) rotateY(360deg)}}
          @keyframes shadow {0%,100%{transform:scaleX(1);opacity:.25}50%{transform:scaleX(.6) translateX(10px);opacity=.1}}
          .card{fill:url(#grad);rx:8;ry:8;transform-origin:80px 50px;animation:flip var(--t) cubic-bezier(.4,.2,.2,1) infinite}
          .stripe{fill:#ffffff}
          .shadow{fill:#000;filter:blur(4px);transform-origin:80px 80px;animation:shadow var(--t) ease-in-out infinite}
        </style>	
        <defs>
          <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#000"/>
            <stop offset="100%" stop-color="#222"/>
          </linearGradient>
        </defs>
        <ellipse class="shadow" cx="80" cy="82" rx="36" ry="8" />
        <rect class="card" x="40" y="25" width="80" height="50" rx="6" ry="6" />
      </svg>
      
    </div>
  </Modal>
{/if} 