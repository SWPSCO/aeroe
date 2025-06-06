<script lang="ts">
    import { onMount } from "svelte";
    import { poke, peek, set } from "$lib/scripts/wallet-commands";
    import { goto } from "$app/navigation";

    // Original state
    let phrase: string[] = $state([]);
    let result = $state(null);
    let attempt = $state(0);
    let peekError = $state(null);
    let initError = $state(null);
    let loading = $state(false);

    // New state for the verification flow
    let step = $state<'display' | 'verify'>('display');
    let userInputs: string[] = $state(Array(24).fill(''));
    let verificationError = $state<string | null>(null);

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

    const handleVerification = async () => {
        verificationError = null;
        const originalPhrase = phrase.join(' ').toLowerCase();
        const enteredPhrase = userInputs.map(w => w.trim()).join(' ').toLowerCase();

        if (originalPhrase !== enteredPhrase) {
            verificationError = "The entered phrase does not match. Please check your words and try again.";
            return;
        }
        
        // If verification is successful, proceed to initialize wallet
        await goToWallet();
    }

    const goToWallet = async () => {
        loading = true;
        const result = await set.initialize();
        if (result.success) {
            goto('/wallet');
        } else {
            initError = result.error;
            loading = false;
        }
    }

    onMount(async () => {
        await handleKeygen();
    });
</script>

<div class="flex flex-col gap-8 items-center justify-center">
    {#if loading}
        <h1 class="text-2xl font-title animate-pulse">Verifying and Initializing Wallet...</h1>
    {:else if phrase.length === 0 && !peekError}
        <h1 class="text-2xl font-title animate-pulse">Generating Keys...</h1>
        {#if attempt > 0}
            <h1 class="text-md font-title">Attempt {attempt + 1}</h1>
        {/if}
    {:else}
        {#if peekError}
            <h1 class="text-md font-title text-red-500">Failed to display seedphrase: {peekError}</h1>
        {/if}
        {#if initError}
            <h1 class="text-md font-title text-red-500">Failed to initialize wallet: {initError}</h1>
        {/if}

        {#if step === 'display'}
            <h1 class="text-2xl font-title">Your Recovery Phrase</h1>
            <p class="text-md font-bold text-red-600 text-center max-w-2xl">
                Write down this phrase in the correct order and store it in a safe, offline location.
                Never share this phrase with anyone. Anyone with this phrase can access your funds.
            </p>
            <div class="grid grid-cols-6 gap-4">
                {#each phrase as word, index}
                    <p class="text-sm font-title text-dark border border-dark p-4 text-center w-[180px]">{index + 1}. {word}</p>
                {/each}
            </div>
            <button class="bg-dark text-white py-4 px-8" onclick={() => step = 'verify'}>I Have Saved My Phrase</button>
        {:else if step === 'verify'}
            <h1 class="text-2xl font-title">Verify Your Phrase</h1>
            <p class="text-md font-title">To confirm, please enter your 24-word seed phrase.</p>
            
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4 w-full max-w-4xl px-4">
                {#each { length: 24 } as _, index}
                    <div class="flex items-center gap-2">
                        <span class="text-sm font-title text-dark w-6 text-right">{index + 1}.</span>
                        <input
                            type="text"
                            bind:value={userInputs[index]}
                            class="text-sm font-title text-dark border border-dark p-2 text-center w-full focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            placeholder={`Word ${index + 1}`}
                            aria-label={`Word ${index + 1} of recovery phrase`}
                        />
                    </div>
                {/each}
            </div>

            {#if verificationError}
                <p class="text-red-500 text-center max-w-md">{verificationError}</p>
            {/if}

            <div class="flex gap-4">
                <button class="bg-gray-300 text-dark py-4 px-8" onclick={() => step = 'display'}>Back to Phrase</button>
                <button class="bg-dark text-white py-4 px-8" onclick={handleVerification}>Verify & Finish</button>
            </div>
        {/if}
    {/if}
</div>

<style>
    /* Ensure consistent input field sizing and responsiveness */
    input {
        min-width: 100px; /* Adjust as needed */
    }
</style>
