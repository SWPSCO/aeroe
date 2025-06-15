<script lang="ts">
    import { walletStore } from '$lib/stores/wallet';
	import TransactionList from '$lib/components/wallet/TransactionList.svelte';
	import SendForm from '$lib/components/wallet/SendForm.svelte';
	import QrCode from '$lib/components/wallet/QrCode.svelte';

	let showSendForm = false;
	let showReceive = false;
</script>

<div class="w-full flex flex-col gap-8 p-8">
	{#if $walletStore.balance}
		<div class="px-8 pt-8 pb-4">
			<div class="uppercase font-title text-dark text-sm">Total Assets Value</div>
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
						showSendForm = !showSendForm;
						showReceive = false;
					}}
					class="flex-1 p-4 bg-dark text-white font-title"
				>
					Send
				</button>
				<button
					on:click={() => {
						showReceive = !showReceive;
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
			<h2 class="font-title text-xl mb-4">Your Address</h2>
			{#if $walletStore.masterPubkey}
				<QrCode address={$walletStore.masterPubkey} />
				<div class="mt-4 p-4 bg-light border-2 border-dark break-all font-mono w-full max-w-lg text-center">
					{$walletStore.masterPubkey}
				</div>
			{:else}
				<p class="text-red-500">Could not load wallet address.</p>
			{/if}
		</div>
	{/if}

	<div class="px-8 pb-8">
		<TransactionList transactions={$walletStore.transactions} />
	</div>
</div> 