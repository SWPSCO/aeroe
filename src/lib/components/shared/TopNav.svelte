<script>
    import Desktop from './TopNav/Desktop.svelte';
    import Mobile from './TopNav/Mobile.svelte';

    $: innerWidth = 0;

</script>
<svelte:window bind:innerWidth/>
{#if innerWidth > 768}
    <Desktop />
{:else}
    <Mobile />
{/if}