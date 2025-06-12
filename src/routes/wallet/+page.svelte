<script lang="ts">
    import { walletStore } from '$lib/stores/wallet';
</script>

<div class="p-8">
    <h1 class="text-2xl font-title">Wallet Overview</h1>
    <p class="mt-2 text-gray-600">This is your main wallet dashboard.</p>

    {#if $walletStore.status === 'loaded' && $walletStore.balance}
        <div class="mt-8 p-6 border rounded-lg bg-white">
            <h2 class="font-title text-lg">Your Balance</h2>
            <div class="text-4xl font-bold mt-2">
                {$walletStore.balance.amount} <span class="text-2xl text-gray-500">{$walletStore.balance.coin}</span>
            </div>
        </div>
    {/if}
</div> 