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

    onMount(() => {
        // Dev
        if (PUBLIC_AEROE_DEV_PAGE === 'true') {
            goto("/dev");
        }
        // End Dev
    });
</script>

<div class="m-8 flex flex-col h-[calc(100vh-4rem)]">
	{#if $onboardingStore.error}
		<div class="font-title text-xl bg-red-500 text-white p-8">
			Error: {$onboardingStore.error}
		</div>
	{/if}

	<h1 class="font-title text-2xl text-center mb-4">Welcome to Aeroe</h1>
	<p class="text-center mb-6">Before you get started, please review and accept the following:</p>

	<div class="flex-grow flex flex-col gap-4 overflow-hidden">
		<!-- Terms of Use -->
		<div class="flex flex-col border-2 border-dark flex-1 overflow-hidden">
			<h2 class="font-title text-md w-full p-2 border-b-2 border-dark text-center">
				Terms of Use
			</h2>
			<div class="overflow-y-auto p-4 font-body">
				<Terms />
			</div>
		</div>

		<!-- Privacy Policy -->
		<div class="flex flex-col border-2 border-dark flex-1 overflow-hidden">
			<h2 class="font-title text-md w-full p-2 border-b-2 border-dark text-center">
				Privacy Policy
			</h2>
			<div class="overflow-y-auto p-4 font-body">
				<PrivacyPolicy />
			</div>
		</div>
	</div>

	<!-- Acceptance -->
	<div class="mt-6 flex-shrink-0">
		<label class="flex items-center gap-4 cursor-pointer p-4 border-2 border-dark">
			<input
				type="checkbox"
				bind:checked={$onboardingStore.termsAccepted}
				class="h-6 w-6"
			/>
			<span>I have read and agree to the Terms of Use.</span>
		</label>
		<label class="mt-2 flex items-center gap-4 cursor-pointer p-4 border-2 border-dark">
			<input
				type="checkbox"
				bind:checked={$onboardingStore.privacyAccepted}
				class="h-6 w-6"
			/>
			<span>I have read and agree to the Privacy Policy.</span>
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