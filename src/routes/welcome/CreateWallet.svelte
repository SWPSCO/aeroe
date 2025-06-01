<script lang="ts">
    import { onMount } from "svelte";
    import { poke, peek, set } from "$lib/scripts/wallet-commands";
    import { goto } from "$app/navigation";

    let phrase: string[] = $state([]);
    let result = $state(null);
    let attempt = $state(0);
    let peekError = $state(null);
    let initError = $state(null);

    const handleKeygen = async () => {
        result = await poke.keygen();
        if (result.success) {
            const phraseResult = await peek.seedphrase();
            if (phraseResult.success) {
                phrase = phraseResult.data;
            } else {
                peekError = phraseResult.error;
            }
        } else {
            attempt++;
            await handleKeygen();
        }
    }


    const goToWallet = async () => {
        const result = await set.initialize();
        if (result.success) {
            goto('/wallet');
        } else {
            initError = result.error;
        }
    }

    onMount(async () => {
        await handleKeygen();
    });
</script>

<div class="flex flex-col gap-8 items-center justify-center">
    {#if phrase.length === 0 && !peekError}
        <h1 class="text-2xl font-title animate-pulse">Generating Keys...</h1>
        {#if attempt > 0}
            <h1 class="text-md font-title">Attempt {attempt + 1}</h1>
        {/if}
    {:else}
        {#if peekError}
            <h1 class="text-md font-title">Failed to display seedphrase: {peekError}</h1>
        {/if}
        {#if initError}
            <h1 class="text-md font-title">Failed to initialize wallet: {initError}</h1>
        {/if}
        <div class="grid grid-cols-6 gap-4">
            {#each phrase as word, index}
                <p class="text-sm font-title text-dark border border-dark p-4 text-center w-[180px]">{index + 1}. {word}</p>
            {/each}
        </div>
        <h1 class="text-md font-title">Save your phrase in a safe place</h1>
        <button class="bg-dark text-white py-4 px-8" onclick={goToWallet}>Go to wallet</button>
    {/if}
</div>