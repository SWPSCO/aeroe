<script lang="ts">
    import { page } from '$app/stores'
    export let history = []
    export let max = 0
    $: width = 0
    $: collapsed = width < 800

    $: transactions = history?.sort((a, b) => b.date - a.date) || []
    $: groupedTransactions = transactions.reduce((acc, transaction) => {
        const date = transaction.date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
        if (!acc[date]) acc[date] = [];
        acc[date].push(transaction);
        return acc;
    }, {})
    $: displayTransactions = max > 0 ? transactions.slice(0, max) : transactions

    $: highlight = (txid) => {
        if (txid == $page.url.hash.replace("#", "")) {
            return "border-dark font-bold"
        } else {
            return ""
        }
    }

</script>
<div bind:clientWidth={width} class="p-8 bg-light flex flex-col gap-8 mb-8 border-2 border-dark">
    {#if displayTransactions.length > 0}
        {#each Object.entries(groupedTransactions) as [date, dayTransactions]}
            {#if dayTransactions.some(t => displayTransactions.includes(t))}
                <div class="mb-2">
                    <h3 class="text-sm font-semibold font-title uppercase">{date}</h3>
                    <div class="opacity-50 text-xs font-title uppercase grid grid-cols-12 items-center py-2">
                        <p class="col-span-1">time</p>
                        <p class="col-span-1">status</p>
                        <p class="col-span-4">sender</p>
                        <p class="col-span-4">recipient</p>
                        <p class="col-span-2 text-right">amount</p>
                    </div>
                    {#each dayTransactions as transaction}
                        {#if displayTransactions.includes(transaction)}
                            <div class="text-xs font-title uppercase grid grid-cols-12 items-center py-4 border-b border-[#B8B8B8] {highlight(transaction.txid)}" id={transaction.txid}>
                                <p class="col-span-1">{transaction.date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })}</p>
                                <p class="col-span-1 text-highlight-orange">{transaction.type}</p>
                                <p class="col-span-4">{transaction.txid}</p>
                                <p class="col-span-4">{transaction.address}</p>
                                <p class="col-span-2 text-right">{transaction.amount.toFixed(2)}</p>
                            </div>
                        {/if}
                    {/each}
                </div>
            {/if}
        {/each}
    {/if}
</div>

<style>
</style>