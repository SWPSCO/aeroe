<script lang="ts">
    import { poke, peek, set } from '$lib/scripts/wallet-commands';
    import { goto } from '$app/navigation';

    let phrase: string[] = $state(Array(24).fill(''));
    let loading = $state(false);
    let importError = $state(null);

    const importKeys = async () => {
        loading = true;
        const result = await poke.genMasterPrivkey(phrase);
        if (result.success) {
            const seedphraseResult = await peek.seedphrase();
            if (seedphraseResult.success) {
                const statePhrase = seedphraseResult.data;
                // compare statePhrase with phrase
                if (statePhrase.join(" ") === phrase.join(" ")) {
                    // initialize wallet
                    const result = await set.initialize();
                    if (result.success) {
                        loading = false;
                        goto('/wallet');
                    } else {
                        importError = result.error;
                    }
                } else {
                    importError = "Invalid seedphrase";
                }
            } else {
                importError = seedphraseResult.error;
            }
        } else {
            loading = false;
            importError = result.error;
            setTimeout(() => {
                importError = null;
            }, 5000);
        }
    }
</script>

<div class="flex flex-col gap-8 items-center justify-center">
    {#if loading}
        <div class="animate-pulse text-2xl font-title">Importing wallet...</div>
    {:else}
        <h1 class="text-2xl font-title">Import Wallet</h1>
        <p class="text-md font-title">Enter your 24-word recovery phrase.</p>
        
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4 w-full max-w-4xl px-4">
            {#each { length: 24 } as _, index}
                <div class="flex items-center gap-2">
                    <span class="text-sm font-title text-dark w-6 text-right">{index + 1}.</span>
                    <input
                        type="text"
                        bind:value={phrase[index]}
                        class="text-sm font-title text-dark border border-dark p-2 text-center w-full focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        placeholder={`Word ${index + 1}`}
                        aria-label={`Word ${index + 1} of recovery phrase`}
                    />
                </div>
            {/each}
        </div>

        <button 
            class="bg-dark text-white py-4 px-8 hover:bg-gray-700 transition-colors duration-150"
            onclick={importKeys}
        >
            Import Wallet
        </button>
    {/if}
    {#if importError}
        <div class="text-red-500">{importError}</div>
    {/if}
</div>

<style>
    /* Ensure consistent input field sizing and responsiveness */
    input {
        min-width: 100px; /* Adjust as needed */
    }
</style>