<script>
    import Header from './Header.svelte';
    import Chart from './Chart.svelte';
    import Send from "./Send.svelte"
    import Title from "./Title.svelte"
    import ListHistory from "./ListHistory.svelte"
    import ReceiveMVP from "./ReceiveMVP.svelte"

    let history = $state([])
</script>

<div class="flex flex-col bg-light">
    <Header />
    <Chart />
     <div class="flex flex-col p-4">
        <Title>send transaction</Title>
        <Send />
        <Title>receive transaction</Title>
        <ReceiveMVP />
        <Title>history</Title>
        {#if history.length > 0}
            <ListHistory {history} />
        {:else}
            <div class="text-center text-sm font-title border-2 border-dark p-8">
                no transaction history yet
            </div>
        {/if}
    </div>
</div>