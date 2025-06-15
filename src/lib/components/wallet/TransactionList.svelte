<script lang="ts">
	import type { NockchainTxMeta } from '$lib/services/tauri';
	import { walletStore } from '$lib/stores/wallet';
	import { sessionStore } from '$lib/stores/session';

	let { transactions } = $props<{ transactions: { [draftId: string]: NockchainTxMeta } }>();

	function parseNano(timestamp: string): Date {
		// Convert nanosecond string to a number, then to milliseconds for the Date constructor
		return new Date(Number(timestamp) / 1000000);
	}

	const transactionList = $derived(
		(Object.values(transactions) as NockchainTxMeta[])
			.filter((tx) => tx.status !== 'draft')
			.sort((a, b) => parseNano(b.createdAt).getTime() - parseNano(a.createdAt).getTime())
	);

	function handleSign(draftId: string) {
		if ($sessionStore.activeWalletName) {
			walletStore.signTransaction(draftId, $sessionStore.activeWalletName);
		}
	}

	function handleSend(draftId: string) {
		if ($sessionStore.activeWalletName) {
			walletStore.sendTransaction(draftId, $sessionStore.activeWalletName);
		}
	}
</script>

<div class="w-full flex flex-col gap-2">
	<h3 class="font-title text-lg">Transaction History</h3>
	{#if transactionList.length > 0}
		<div class="flex flex-col gap-2">
			{#each transactionList as tx}
				<div class="p-4 border-2 border-dark flex justify-between items-center">
					<div>
						<p class="font-bold">{tx.status.toUpperCase()}</p>
						<p class="text-sm text-gray-500">
							To: {tx.transactions[0]?.recipient.substring(0, 24)}...
						</p>
					</div>
					<div class="text-right">
						<p class="font-bold">
							{tx.transactions.reduce((acc, curr) => acc + curr.amount, 0)} Nock
						</p>
						<p class="text-sm text-gray-500">{parseNano(tx.createdAt).toLocaleString()}</p>
					</div>
					{#if tx.status === 'draft'}
						<button on:click={() => handleSign(tx.draftId)} class="p-2 bg-blue-500 text-white font-title text-sm">
							Sign
						</button>
					{:else if tx.status === 'signed'}
						<button on:click={() => handleSend(tx.draftId)} class="p-2 bg-green-500 text-white font-title text-sm">
							Send
						</button>
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<div class="p-8 border-2 border-dark text-center text-gray-500">
			No transactions yet.
		</div>
	{/if}
</div> 