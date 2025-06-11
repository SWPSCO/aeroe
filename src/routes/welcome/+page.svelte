<script lang="ts">
    import CreatePassword from './CreatePassword.svelte';
    import Splash from './Splash.svelte';
    import CreateWallet from './CreateWallet.svelte';
    import ImportWallet from './ImportWallet.svelte';

    let currentPage = $state("create-password");
</script>

<div class="h-screen w-screen flex flex-col items-center justify-center gap-8">
    {#if currentPage === "create-password"}
        <CreatePassword on:created={() => currentPage = "splash"} />
    {:else if currentPage === "splash"}
        <Splash 
            on:create={() => currentPage = "create"}
            on:import={() => currentPage = "import"}
        />
    {:else if currentPage === "create"}
        <CreateWallet />
    {:else if currentPage === "import"}
        <ImportWallet />
    {/if}
</div>