import { writable } from 'svelte/store';
import { mainStore } from './main';
import { terms } from '$lib/services';

interface OnboardingState {
    termsAccepted: boolean;
    privacyAccepted: boolean;
    error: string | null;
}

function createOnboardingStore() {
    const store = writable<OnboardingState>({
        termsAccepted: false,
        privacyAccepted: false,
        error: null,
    });
    const { subscribe, update } = store;

    function toggleTerms() {
        update(s => ({ ...s, termsAccepted: !s.termsAccepted, error: null }));
    }

    function togglePrivacy() {
        update(s => ({ ...s, privacyAccepted: !s.privacyAccepted, error: null }));
    }

    async function submit() {
        try {
            await terms.setTermsAccepted();
            await terms.setPrivacyAccepted();
            mainStore.completeOnboarding();
        } catch (e: any) {
            update(s => ({ ...s, error: e.message || 'An unknown error occurred.'}))
        }
    }

    return {
        subscribe,
        toggleTerms,
        togglePrivacy,
        submit,
    }
}

export const onboardingStore = createOnboardingStore(); 