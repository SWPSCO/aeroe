<script>
    import Desktop from '$lib/components/TopNav/Desktop.svelte';
    import Mobile from '$lib/components/TopNav/Mobile.svelte';

    $: innerWidth = 0;

</script>
<svelte:window bind:innerWidth/>
{#if innerWidth > 768}
    <Desktop />
{:else}
    <Mobile />
{/if}