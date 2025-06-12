<script lang="ts">
    import { onMount } from 'svelte';
    import { onboardingStore } from '$lib/stores/onboarding';
    import TermsOfUse from './TermsOfUse.svelte';
    import PrivacyPolicy from './PrivacyPolicy.svelte';

    onMount(() => {
        onboardingStore.checkStatus();
    });
</script>

{#if $onboardingStore.error}
    <div class="m-8">
        <div class="font-title text-xl bg-red-500 text-white p-8">
            Error: {$onboardingStore.error}
        </div>
    </div>
{:else if !$onboardingStore.termsAccepted || !$onboardingStore.privacyAccepted}
    <div class="m-8">
        <div class="font-title text-md w-full p-2 border-2 border-dark text-center">
            {$onboardingStore.termsAccepted ? "Privacy Policy" : "Terms of Use"}
        </div>
        <div class="w-full overflow-y-auto p-4 font-title border-2 border-dark border-t-0 h-[680px] flex flex-col gap-4"> 
            {#if $onboardingStore.termsAccepted}
                <PrivacyPolicy />
            {:else}
                <TermsOfUse />
            {/if}
        </div>
        <button 
            class="border-2 border-dark w-full p-4 mt-2 font-title bg-dark text-white cursor-pointer" 
            onclick={() => { $onboardingStore.termsAccepted ? onboardingStore.acceptPrivacy() : onboardingStore.acceptTerms() }}
        >
            Accept
        </button>
    </div>
{:else}
    <!-- Both accepted, the store is handling navigation. Show a loading state. -->
    <div class="flex justify-center items-center h-screen">
        <div class="animate-pulse text-2xl font-title">Loading...</div>
    </div>
{/if}