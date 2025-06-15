<script lang="ts">
	import { walletStore } from '$lib/stores/wallet';
	import { sessionStore } from '$lib/stores/session';

	let recipient = '';
	let amount: number | null = null;
	let fee: number | null = 10;
	
	let state: 'composing' | 'confirming' | 'sending' = 'composing';
	let draftId: string | null = null;

	async function createDraft() {
		if (!recipient || !amount || !fee || !$sessionStore.activeWalletName) {
			walletStore.setError('Please fill out all fields.');
			return;
		}
		walletStore.clearError();
		state = 'sending';
		
		await walletStore.createTransaction(recipient, amount, fee, $sessionStore.activeWalletName);
		
		if ($walletStore.error) {
			state = 'composing';
		} else {
			// Find the newly created draft to get its ID
			const drafts = Object.values($walletStore.transactions);
			const newDraft = drafts.find(d => d.status === 'draft' && !d.signed_at);
			if (newDraft) {
				draftId = newDraft.draftId;
				state = 'confirming';
			} else {
				walletStore.setError('Could not find created draft.');
				state = 'composing';
			}
		}
	}

	async function signAndSend() {
		if (!draftId || !$sessionStore.activeWalletName) {
			walletStore.setError('No transaction draft to send.');
			return;
		}
		walletStore.clearError();
		state = 'sending';

		await walletStore.signTransaction(draftId, $sessionStore.activeWalletName);
		if ($walletStore.error) {
			state = 'confirming';
			return;
		}

		await walletStore.sendTransaction(draftId, $sessionStore.activeWalletName);
		if ($walletStore.error) {
			state = 'confirming';
			return;
		}

		state = 'composing';
		recipient = '';
		amount = null;
		fee = 10;
		draftId = null;
	}
</script>

<div class="flex flex-col gap-6 p-6 border-2 border-dark bg-white">
	{#if state === 'composing'}
		<h3 class="font-title text-lg">Compose Transaction</h3>
		<div>
			<label for="recipient" class="block font-title text-sm">Recipient Address</label>
			<input type="text" id="recipient" bind:value={recipient} class="mt-1 block w-full border-2 border-dark p-2" placeholder="Enter Nock address..." />
		</div>
		<div class="flex gap-4">
			<div class="flex-1">
				<label for="amount" class="block font-title text-sm">Amount</label>
				<input type="number" id="amount" bind:value={amount} class="mt-1 block w-full border-2 border-dark p-2" placeholder="0.0" />
			</div>
			<div class="w-1/3">
				<label for="fee" class="block font-title text-sm">Fee</label>
				<input type="number" id="fee" bind:value={fee} class="mt-1 block w-full border-2 border-dark p-2" />
			</div>
		</div>
		<button on:click={createDraft} class="p-4 bg-dark text-white font-title">Create Draft</button>

	{:else if state === 'confirming'}
		<h3 class="font-title text-lg">Confirm Transaction</h3>
		<div class="flex flex-col gap-4">
			<div>
				<p class="font-title text-sm text-gray-500">Amount</p>
				<p class="text-xl font-bold">{amount} Nock</p>
			</div>
			<div>
				<p class="font-title text-sm text-gray-500">To</p>
				<p class="font-mono text-sm break-all">{recipient}</p>
			</div>
			<div>
				<p class="font-title text-sm text-gray-500">Fee</p>
				<p class="font-mono text-sm">{fee} Nock</p>
			</div>
		</div>
		<button on:click={signAndSend} class="p-4 bg-green-600 text-white font-title">Sign and Send</button>

	{:else if state === 'sending'}
		<div class="p-8 text-center">
			<p class="animate-pulse font-title">Sending...</p>
		</div>
	{/if}

	{#if $walletStore.error}
		<div class="p-2 bg-red-500 text-white text-sm">
			{$walletStore.error}
		</div>
	{/if}
</div> 