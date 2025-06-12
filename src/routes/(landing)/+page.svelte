<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { terms, aeroe } from '$lib/scripts/commands';
    import Terms from './Terms.svelte';
    import PrivacyPolicy from './PrivacyPolicy.svelte';

    let pageIsReady = $state(false);
    let termsOfUseAccepted = $state(false);
    let privacyPolicyAccepted = $state(false);
    
    $effect(() => {
        async function handleNavigation() {
            if (termsOfUseAccepted && privacyPolicyAccepted) {
                const statusResult = await aeroe.status();
                if (statusResult.success) {
                    if (statusResult.data.vaultExists) {
                        goto("/login");
                    } else {
                        goto("/welcome");
                    }
                } else {
                    error = `Failed to check vault status: ${JSON.stringify(statusResult.error)}`;
                }
            }
        }
        handleNavigation();
    });

    let error: unknown | null = $state(null);

    const checkTermsAccepted = async () => {
        const termsAccepted = await terms.isTermsAccepted();
        if (termsAccepted.success) {
            termsOfUseAccepted = termsAccepted.data;
        } else {
            error = termsAccepted.error;
        }
    }

    const checkPrivacyAccepted = async () => {
        const privacyAccepted = await terms.isPrivacyAccepted();
        if (privacyAccepted.success) {
            privacyPolicyAccepted = privacyAccepted.data;
        } else {
            error = privacyAccepted.error;
        }
    }

    const acceptTermsOfUse = async () => {
        await terms.setTermsAccepted();
        await checkTermsAccepted();
    }
    const acceptPrivacyPolicy = async () => {
        await terms.setPrivacyAccepted();
        await checkPrivacyAccepted();
    }

    onMount(async () => {
        await checkTermsAccepted();
        await checkPrivacyAccepted();
        if (!termsOfUseAccepted || !privacyPolicyAccepted) {
            pageIsReady = true;
        }
    });

</script>
{#if error}
    <div class="m-8">
        <div class="font-title text-xl bg-red-500 text-white p-8">
            Error: {JSON.stringify(error)}
        </div>
    </div>
{/if}
{#if pageIsReady}
<div class="m-8">
    <div class="font-title text-md w-full p-2 border-2 border-dark text-center">
        {termsOfUseAccepted ? "Privacy Policy" : "Terms of Use"}
    </div>
    <div class="w-full overflow-y-auto p-4 font-title border-2 border-dark border-t-0 h-[680px] flex flex-col gap-4"> 
        {#if termsOfUseAccepted}
            <PrivacyPolicy />
        {:else}
            <Terms />
        {/if}
    </div>
    <button class="border-2 border-dark w-full p-4 mt-2 font-title bg-dark text-white cursor-pointer" onclick={()=>{termsOfUseAccepted ? acceptPrivacyPolicy() : acceptTermsOfUse()}}>Accept</button>
</div>
{/if}