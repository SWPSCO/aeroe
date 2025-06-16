<script lang="ts">
    import { onMount } from 'svelte';
    import { onboardingStore } from '$lib/stores/onboarding';
    import Terms from './Terms.svelte';
    import PrivacyPolicy from './PrivacyPolicy.svelte';

    // Dev
    import { PUBLIC_AEROE_DEV_PAGE } from '$env/static/public';
    import { goto } from '$app/navigation';
    // End Dev
    
    $: allAccepted = $onboardingStore.termsAccepted && $onboardingStore.privacyAccepted;

    type OpenSection = 'terms' | 'privacy' | 'none';
    let openSection: OpenSection = 'terms'; // Default to terms open

    onMount(() => {
        // Dev
        if (PUBLIC_AEROE_DEV_PAGE === 'true') {
            goto("/dev");
        }
        // End Dev
    });
</script>

<div class="m-8 flex flex-col flex-grow min-h-0 bg-light">
	{#if $onboardingStore.error}
		<div class="font-title text-xl bg-red-500 text-white p-8">
			Error: {$onboardingStore.error}
		</div>
	{/if}

	<div class="flex-grow min-h-0 flex flex-col gap-4">
		<!-- Terms of Use Accordion -->
		<div class={`flex flex-col border-2 border-dark ${openSection === 'terms' ? 'flex-grow min-h-0' : ''}`}>
			<button on:click={() => openSection = openSection === 'terms' ? 'none' : 'terms'} class="font-title text-md w-full p-4 flex items-center justify-between">
				<span>Terms of Use</span>
				<svg xmlns="http://www.w3.org/2000/svg" class={`w-5 h-5 transition-transform ${openSection === 'terms' ? 'rotate-180' : ''}`} fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
				</svg>
			</button>
			{#if openSection === 'terms'}
				<div class="flex-grow overflow-y-auto p-4 font-body border-t-2 border-dark custom-scrollbar">
					<Terms />
				</div>
			{/if}
		</div>

		<!-- Privacy Policy Accordion -->
		<div class={`flex flex-col border-2 border-dark mt-4 ${openSection === 'privacy' ? 'flex-grow min-h-0' : ''}`}>
			<button on:click={() => openSection = openSection === 'privacy' ? 'none' : 'privacy'} class="font-title text-md w-full p-4 flex items-center justify-between">
				<span>Privacy Policy</span>
				<svg xmlns="http://www.w3.org/2000/svg" class={`w-5 h-5 transition-transform ${openSection === 'privacy' ? 'rotate-180' : ''}`} fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
				</svg>
			</button>
			{#if openSection === 'privacy'}
				<div class="flex-grow overflow-y-auto p-4 font-body border-t-2 border-dark custom-scrollbar">
					<PrivacyPolicy />
				</div>
			{/if}
		</div>
	</div>

	<!-- Acceptance -->
	<div class="mt-6 flex-shrink-0">
		<label class="flex items-center gap-4 cursor-pointer">
			<input
				type="checkbox"
				class="h-6 w-6 border-2 border-dark"
				checked={$onboardingStore.termsAccepted}
				on:click={onboardingStore.toggleTerms}
			/>
			<span class="text-sm">I have read and agree to the Terms of Use.</span>
		</label>
		<label class="mt-2 flex items-center gap-4 cursor-pointer">
			<input
				type="checkbox"
				class="h-6 w-6 border-2 border-dark"
				checked={$onboardingStore.privacyAccepted}
				on:click={onboardingStore.togglePrivacy}
			/>
			<span class="text-sm">I have read and agree to the Privacy Policy.</span>
		</label>

		<button
			class="border-2 border-dark w-full p-4 mt-4 font-title bg-dark text-white disabled:bg-medium disabled:cursor-not-allowed"
			disabled={!allAccepted}
			on:click={onboardingStore.submit}
		>
			Accept and Continue
		</button>
	</div>
</div>