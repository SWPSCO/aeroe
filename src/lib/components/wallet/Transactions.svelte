<script>
    import TransactionCard from './TransactionCard.svelte';
    $: width = 0;
    $: collapsed = width < 800;
</script>

<div bind:clientWidth={width} class="bg-light flex flex-col p-4 border-t-2 border-light">
    {#if !collapsed}
    <div class="grid grid-cols-9 gap-4 p-4 text-xs font-title uppercase text-center font-bold">
        <div class="col-span-2">Transaction Amount</div>
        <div>Status</div>
        <div class="col-span-2">Sender</div>
        <div class="col-span-2">Recipient</div>
        <div>Date & Time</div>
        <div></div>
    </div>
    {:else}
    <div class="flex uppercase font-title">
        <div class="flex-1 font-bold">Recent Transactions:</div>
        <div class="text-highlight-orange underline">view all</div>
    </div>
    {/if}
    {#each Array(10) as _}
        <TransactionCard {collapsed} />
    {/each}
</div>